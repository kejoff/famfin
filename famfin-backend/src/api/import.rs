use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use base64::Engine;

use crate::{
    api::AppState,
    auth::middleware::AuthSession,
    db::queries,
    models::CreateTransactionRequest,
};

#[derive(Serialize, Deserialize)]
pub struct ImportRequest {
    pub transactions: Vec<TransactionImport>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionImport {
    pub date: String,              // YYYY-MM-DD
    pub amount: f64,               // negative for expenses
    pub merchant_name: String,
    pub description: Option<String>,
    pub category_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImportFileRequest {
    pub filename: String,          // For parsing (csv, ofx, qfx)
    pub data: String,              // Base64 encoded file data
}

#[derive(Serialize, Deserialize)]
pub struct ImportResponse {
    pub imported_count: usize,
    pub skipped_count: usize,
    pub errors: Vec<String>,
}

/// Generate fingerprint for transaction deduplication
fn generate_fingerprint(date: &str, amount: f64, merchant: &str, ordinal: usize) -> String {
    let combined = format!("{}|{}|{}|{}", date, amount, merchant.to_lowercase(), ordinal);
    let mut hasher = Sha256::new();
    hasher.update(&combined);
    format!("{:x}", hasher.finalize())
}

pub async fn import_transactions(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<ImportRequest>,
) -> Result<(StatusCode, Json<ImportResponse>), (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    let db = state.db.lock().await;

    let mut imported_count = 0;
    let mut skipped_count = 0;
    let mut errors = Vec::new();

    // Group transactions by date + merchant for fingerprinting
    let mut transaction_groups: std::collections::HashMap<(String, String), Vec<TransactionImport>> =
        std::collections::HashMap::new();

    for txn in req.transactions {
        let key = (txn.date.clone(), txn.merchant_name.clone());
        transaction_groups.entry(key).or_insert_with(Vec::new).push(txn);
    }

    // Process each group
    for ((_date, _merchant), group) in transaction_groups {
        for (ordinal, txn) in group.iter().enumerate() {
            // Validate transaction
            if txn.date.is_empty() || txn.merchant_name.is_empty() {
                errors.push("Transaction missing required fields".to_string());
                skipped_count += 1;
                continue;
            }

            // Generate fingerprint for deduplication
            let fingerprint = generate_fingerprint(&txn.date, txn.amount, &txn.merchant_name, ordinal);

            // Check if already exists
            let stmt_result = db.prepare(
                "SELECT COUNT(*) FROM transactions WHERE household_id = ?1 AND import_fingerprint = ?2"
            );

            match stmt_result {
                Ok(mut stmt) => {
                    let exists: i32 = stmt.query_row([&household_id, &fingerprint], |row| row.get(0))
                        .unwrap_or(0);

                    if exists > 0 {
                        skipped_count += 1;
                        continue;
                    }
                }
                Err(e) => {
                    errors.push(format!("Database check failed: {}", e));
                    skipped_count += 1;
                    continue;
                }
            }

            // Create transaction
            let req = CreateTransactionRequest {
                date: txn.date.clone(),
                amount: txn.amount,
                merchant_name: txn.merchant_name.clone(),
                description: txn.description.clone(),
                category_id: txn.category_id.clone(),
                import_fingerprint: fingerprint,
            };

            match queries::create_transaction(&db, &household_id, &req) {
                Ok(_) => imported_count += 1,
                Err(e) => {
                    errors.push(format!("Failed to create transaction: {}", e));
                    skipped_count += 1;
                }
            }
        }
    }

    Ok((
        StatusCode::OK,
        Json(ImportResponse {
            imported_count,
            skipped_count,
            errors,
        }),
    ))
}

/// Parse CSV file and extract transactions
fn parse_csv(csv_data: &[u8]) -> Result<Vec<TransactionImport>, String> {
    let mut transactions = Vec::new();
    let mut reader = csv::Reader::from_reader(csv_data);

    for (idx, result) in reader.deserialize::<TransactionImport>().enumerate() {
        match result {
            Ok(txn) => transactions.push(txn),
            Err(e) => return Err(format!("CSV parse error at row {}: {}", idx + 1, e)),
        }
    }

    Ok(transactions)
}

/// Parse OFX file and extract transactions (simplified)
fn parse_ofx(ofx_data: &[u8]) -> Result<Vec<TransactionImport>, String> {
    let text = String::from_utf8(ofx_data.to_vec())
        .map_err(|e| format!("Invalid UTF-8 in OFX file: {}", e))?;

    let mut transactions = Vec::new();

    // Simple OFX parser - look for STMTTRN blocks
    for block in text.split("<STMTTRN>") {
        if !block.contains("</STMTTRN>") {
            continue;
        }

        let block = block.split("</STMTTRN>").next().unwrap_or("");

        // Extract fields
        let dtposted = extract_ofx_field(block, "DTPOSTED").unwrap_or_default();
        let trnamt = extract_ofx_field(block, "TRNAMT").unwrap_or_default();
        let name = extract_ofx_field(block, "NAME").unwrap_or_default();
        let memo = extract_ofx_field(block, "MEMO").unwrap_or_default();

        let amount = trnamt.parse::<f64>().unwrap_or(0.0);

        // Format date YYYYMMDD -> YYYY-MM-DD
        let date = if dtposted.len() >= 8 {
            format!("{}-{}-{}", &dtposted[0..4], &dtposted[4..6], &dtposted[6..8])
        } else {
            continue;
        };

        transactions.push(TransactionImport {
            date,
            amount,
            merchant_name: name,
            description: if memo.is_empty() { None } else { Some(memo) },
            category_id: None,
        });
    }

    Ok(transactions)
}

/// Extract OFX field value (e.g., <FIELDNAME>value</FIELDNAME>)
fn extract_ofx_field(block: &str, field: &str) -> Option<String> {
    let start_tag = format!("<{}>", field);
    let end_tag = format!("</{}>", field);

    block
        .find(&start_tag)
        .and_then(|start| {
            let start_pos = start + start_tag.len();
            block[start_pos..].find(&end_tag).map(|end| {
                block[start_pos..start_pos + end].trim().to_string()
            })
        })
}

/// Import file (CSV or OFX) encoded as base64
pub async fn import_file(
    State(state): State<AppState>,
    Path(household_id): Path<String>,
    Extension(auth): Extension<AuthSession>,
    Json(req): Json<ImportFileRequest>,
) -> Result<(StatusCode, Json<ImportResponse>), (StatusCode, String)> {
    if auth.household_id != household_id {
        return Err((StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    // Decode base64 data
    let data = base64_decode(&req.data)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid base64: {}", e)))?;

    // Parse based on file extension
    let transactions = if req.filename.ends_with(".csv") {
        parse_csv(&data)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("CSV parse error: {}", e)))?
    } else if req.filename.ends_with(".ofx") || req.filename.ends_with(".qfx") {
        parse_ofx(&data)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("OFX parse error: {}", e)))?
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Unsupported file type: {}", req.filename),
        ));
    };

    // Import transactions
    let import_req = ImportRequest { transactions };
    import_transactions(State(state), Path(household_id), Extension(auth), Json(import_req))
        .await
}

/// Decode base64 data using standard library
fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("Invalid base64: {}", e))
}
