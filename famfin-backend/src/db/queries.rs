use rusqlite::{Connection, params, OptionalExtension};
use anyhow::Result;
use crate::models::*;

// Transactions
pub fn create_transaction(
    conn: &Connection,
    household_id: &str,
    req: &CreateTransactionRequest,
) -> Result<Transaction> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    conn.execute(
        "INSERT INTO transactions (id, household_id, date, amount, merchant_name, category_id, description, category_source, one_time_flag, import_fingerprint, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            &id,
            household_id,
            &req.date,
            req.amount,
            &req.merchant_name,
            &req.category_id,
            &req.description,
            "uncategorized",
            false,
            &req.import_fingerprint,
            &now,
            &now,
        ],
    )?;

    Ok(Transaction {
        id,
        household_id: household_id.to_string(),
        date: req.date.clone(),
        amount: req.amount,
        merchant_name: req.merchant_name.clone(),
        category_id: req.category_id.clone(),
        description: req.description.clone(),
        category_source: "uncategorized".to_string(),
        one_time_flag: false,
        import_fingerprint: req.import_fingerprint.clone(),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn get_transaction(conn: &Connection, id: &str) -> Result<Option<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, date, amount, merchant_name, category_id, description, category_source, one_time_flag, import_fingerprint, created_at, updated_at
         FROM transactions WHERE id = ?1"
    )?;

    let result = stmt.query_row(params![id], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            household_id: row.get(1)?,
            date: row.get(2)?,
            amount: row.get(3)?,
            merchant_name: row.get(4)?,
            category_id: row.get(5)?,
            description: row.get(6)?,
            category_source: row.get(7)?,
            one_time_flag: row.get::<_, i32>(8)? != 0,
            import_fingerprint: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }).optional()?;

    Ok(result)
}

pub fn list_transactions(
    conn: &Connection,
    household_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, date, amount, merchant_name, category_id, description, category_source, one_time_flag, import_fingerprint, created_at, updated_at
         FROM transactions WHERE household_id = ?1 ORDER BY date DESC LIMIT ?2 OFFSET ?3"
    )?;

    let transactions = stmt.query_map(params![household_id, limit, offset], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            household_id: row.get(1)?,
            date: row.get(2)?,
            amount: row.get(3)?,
            merchant_name: row.get(4)?,
            category_id: row.get(5)?,
            description: row.get(6)?,
            category_source: row.get(7)?,
            one_time_flag: row.get::<_, i32>(8)? != 0,
            import_fingerprint: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(transactions)
}

pub fn update_transaction(
    conn: &Connection,
    id: &str,
    req: &UpdateTransactionRequest,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    // Get current transaction to preserve unchanged fields
    let current = get_transaction(conn, id)?
        .ok_or_else(|| anyhow::anyhow!("Transaction not found"))?;

    let category_id = req.category_id.clone().or(current.category_id);
    let description = req.description.clone().or(current.description);
    let one_time_flag = req.one_time_flag.unwrap_or(current.one_time_flag);

    conn.execute(
        "UPDATE transactions SET category_id = ?1, description = ?2, one_time_flag = ?3, updated_at = ?4 WHERE id = ?5",
        params![
            category_id,
            description,
            if one_time_flag { 1 } else { 0 },
            &now,
            id,
        ],
    )?;

    Ok(())
}

// Categories
pub fn create_category(
    conn: &Connection,
    household_id: &str,
    req: &CreateCategoryRequest,
) -> Result<TransactionCategory> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    conn.execute(
        "INSERT INTO transaction_categories (id, household_id, name, color, icon, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &id,
            household_id,
            &req.name,
            &req.color,
            &req.icon,
            &now,
            &now,
        ],
    )?;

    Ok(TransactionCategory {
        id,
        household_id: household_id.to_string(),
        name: req.name.clone(),
        color: req.color.clone(),
        icon: req.icon.clone(),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn get_category(conn: &Connection, id: &str) -> Result<Option<TransactionCategory>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, name, color, icon, created_at, updated_at
         FROM transaction_categories WHERE id = ?1"
    )?;

    let result = stmt.query_row(params![id], |row| {
        Ok(TransactionCategory {
            id: row.get(0)?,
            household_id: row.get(1)?,
            name: row.get(2)?,
            color: row.get(3)?,
            icon: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }).optional()?;

    Ok(result)
}

pub fn list_categories(conn: &Connection, household_id: &str) -> Result<Vec<TransactionCategory>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, name, color, icon, created_at, updated_at
         FROM transaction_categories WHERE household_id = ?1 ORDER BY name"
    )?;

    let categories = stmt.query_map(params![household_id], |row| {
        Ok(TransactionCategory {
            id: row.get(0)?,
            household_id: row.get(1)?,
            name: row.get(2)?,
            color: row.get(3)?,
            icon: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(categories)
}

// Goals
pub fn create_goal(
    conn: &Connection,
    household_id: &str,
    req: &CreateGoalRequest,
) -> Result<Goal> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    conn.execute(
        "INSERT INTO goals (id, household_id, name, description, target_amount, current_amount, deadline, generates_income, creates_expenses, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            &id,
            household_id,
            &req.name,
            &req.description,
            req.target_amount,
            0.0,
            &req.deadline,
            if req.generates_income.unwrap_or(false) { 1 } else { 0 },
            if req.creates_expenses.unwrap_or(false) { 1 } else { 0 },
            &now,
            &now,
        ],
    )?;

    Ok(Goal {
        id,
        household_id: household_id.to_string(),
        name: req.name.clone(),
        description: req.description.clone(),
        target_amount: req.target_amount,
        current_amount: 0.0,
        deadline: req.deadline.clone(),
        generates_income: req.generates_income.unwrap_or(false),
        creates_expenses: req.creates_expenses.unwrap_or(false),
        metadata: None,
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn list_goals(conn: &Connection, household_id: &str) -> Result<Vec<Goal>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, name, description, target_amount, current_amount, deadline, generates_income, creates_expenses, metadata, created_at, updated_at
         FROM goals WHERE household_id = ?1 ORDER BY deadline"
    )?;

    let goals = stmt.query_map(params![household_id], |row| {
        Ok(Goal {
            id: row.get(0)?,
            household_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            target_amount: row.get(4)?,
            current_amount: row.get(5)?,
            deadline: row.get(6)?,
            generates_income: row.get::<_, i32>(7)? != 0,
            creates_expenses: row.get::<_, i32>(8)? != 0,
            metadata: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(goals)
}

pub fn get_goal(conn: &Connection, id: &str) -> Result<Option<Goal>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, name, description, target_amount, current_amount, deadline, generates_income, creates_expenses, metadata, created_at, updated_at
         FROM goals WHERE id = ?1"
    )?;

    let result = stmt.query_row(params![id], |row| {
        Ok(Goal {
            id: row.get(0)?,
            household_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            target_amount: row.get(4)?,
            current_amount: row.get(5)?,
            deadline: row.get(6)?,
            generates_income: row.get::<_, i32>(7)? != 0,
            creates_expenses: row.get::<_, i32>(8)? != 0,
            metadata: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
        })
    }).optional()?;

    Ok(result)
}

pub fn update_goal(
    conn: &Connection,
    id: &str,
    req: &UpdateGoalRequest,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    // Get current goal to preserve unchanged fields
    let current = get_goal(conn, id)?
        .ok_or_else(|| anyhow::anyhow!("Goal not found"))?;

    let name = req.name.clone().unwrap_or(current.name);
    let description = req.description.clone().or(current.description);
    let target_amount = req.target_amount.unwrap_or(current.target_amount);
    let current_amount = req.current_amount.unwrap_or(current.current_amount);
    let deadline = req.deadline.clone().or(current.deadline);
    let generates_income = req.generates_income.unwrap_or(current.generates_income);
    let creates_expenses = req.creates_expenses.unwrap_or(current.creates_expenses);

    conn.execute(
        "UPDATE goals SET name = ?1, description = ?2, target_amount = ?3, current_amount = ?4, deadline = ?5, generates_income = ?6, creates_expenses = ?7, updated_at = ?8 WHERE id = ?9",
        params![
            &name,
            &description,
            target_amount,
            current_amount,
            &deadline,
            if generates_income { 1 } else { 0 },
            if creates_expenses { 1 } else { 0 },
            &now,
            id,
        ],
    )?;

    Ok(())
}

pub fn delete_goal(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM goals WHERE id = ?1", params![id])?;
    Ok(())
}

// Sessions
pub fn create_session(conn: &Connection, household_id: &str, expires_at: &str) -> Result<Session> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

    conn.execute(
        "INSERT INTO sessions (id, household_id, created_at, expires_at) VALUES (?1, ?2, ?3, ?4)",
        params![&id, household_id, &now, expires_at],
    )?;

    Ok(Session {
        id,
        household_id: household_id.to_string(),
        created_at: now,
        expires_at: expires_at.to_string(),
    })
}

pub fn get_session(conn: &Connection, id: &str) -> Result<Option<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, household_id, created_at, expires_at FROM sessions WHERE id = ?1"
    )?;

    let result = stmt.query_row(params![id], |row| {
        Ok(Session {
            id: row.get(0)?,
            household_id: row.get(1)?,
            created_at: row.get(2)?,
            expires_at: row.get(3)?,
        })
    }).optional()?;

    Ok(result)
}

pub fn delete_session(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM sessions WHERE id = ?1", params![id])?;
    Ok(())
}

// Households
pub fn get_household(conn: &Connection, id: &str) -> Result<Option<Household>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, password_hash, db_version, created_at FROM households WHERE id = ?1"
    )?;

    let result = stmt.query_row(params![id], |row| {
        Ok(Household {
            id: row.get(0)?,
            name: row.get(1)?,
            password_hash: row.get(2)?,
            db_version: row.get(3)?,
            created_at: row.get(4)?,
        })
    }).optional()?;

    Ok(result)
}

// Dashboard aggregations
pub fn get_monthly_spending(
    conn: &Connection,
    household_id: &str,
    year: i32,
    month: u32,
) -> Result<f64> {
    let mut stmt = conn.prepare(
        "SELECT COALESCE(SUM(ABS(amount)), 0) FROM transactions
         WHERE household_id = ?1 AND amount < 0
         AND strftime('%Y', date) = ?2 AND strftime('%m', date) = ?3"
    )?;

    let total: f64 = stmt.query_row(
        params![household_id, format!("{:04}", year), format!("{:02}", month)],
        |row| row.get(0),
    )?;

    Ok(total)
}

pub fn get_category_breakdown(
    conn: &Connection,
    household_id: &str,
    year: i32,
    month: u32,
) -> Result<Vec<(String, String, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT tc.id, tc.name, COALESCE(SUM(ABS(t.amount)), 0) as total
         FROM transactions t
         LEFT JOIN transaction_categories tc ON t.category_id = tc.id
         WHERE t.household_id = ?1 AND t.amount < 0
         AND strftime('%Y', t.date) = ?2 AND strftime('%m', t.date) = ?3
         GROUP BY tc.id, tc.name
         ORDER BY total DESC"
    )?;

    let breakdown = stmt.query_map(
        params![household_id, format!("{:04}", year), format!("{:02}", month)],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, f64>(2)?)),
    )?
    .collect::<Result<Vec<_>, _>>()?;

    Ok(breakdown)
}

pub fn get_average_monthly_spending(
    conn: &Connection,
    household_id: &str,
    months: i32,
) -> Result<f64> {
    let mut stmt = conn.prepare(
        "SELECT COALESCE(AVG(monthly_total), 0)
         FROM (
           SELECT SUM(ABS(amount)) as monthly_total
           FROM transactions
           WHERE household_id = ?1 AND amount < 0
           AND date >= date('now', '-' || ?2 || ' months')
           GROUP BY strftime('%Y-%m', date)
         )"
    )?;

    let avg: f64 = stmt.query_row(params![household_id, months], |row| row.get(0))?;

    Ok(avg)
}
