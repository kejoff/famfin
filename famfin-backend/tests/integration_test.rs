mod common;

use rusqlite::params;
use common::init_test_db;
use famfin_backend::db::queries;
use famfin_backend::models::*;

#[test]
fn test_schema_creation() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Verify households table exists
    let mut stmt = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='households'")
        .expect("Failed to prepare statement");
    let name: String = stmt
        .query_row([], |row| row.get(0))
        .expect("households table not found");
    assert_eq!(name, "households");

    // Verify transactions table exists
    let mut stmt = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='transactions'")
        .expect("Failed to prepare statement");
    let name: String = stmt
        .query_row([], |row| row.get(0))
        .expect("transactions table not found");
    assert_eq!(name, "transactions");

    // Verify sessions table exists
    let mut stmt = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='sessions'")
        .expect("Failed to prepare statement");
    let name: String = stmt
        .query_row([], |row| row.get(0))
        .expect("sessions table not found");
    assert_eq!(name, "sessions");
}

#[test]
fn test_households_table() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Insert a household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Query it back
    let mut stmt = db
        .prepare("SELECT id, name FROM households WHERE id = ?1")
        .expect("Failed to prepare statement");
    let result: (String, String) = stmt
        .query_row(params!["hh-001"], |row| Ok((row.get(0)?, row.get(1)?)))
        .expect("Failed to query household");

    assert_eq!(result.0, "hh-001");
    assert_eq!(result.1, "My Household");
}

#[test]
fn test_sessions_table() {
    let db = init_test_db().expect("Failed to initialize test database");

    // First create a household (foreign key requirement)
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Insert a session
    db.execute(
        "INSERT INTO sessions (id, expires_at, household_id) VALUES (?1, ?2, ?3)",
        params![
            "sess-001",
            "2026-12-31 23:59:59",
            "hh-001"
        ],
    )
    .expect("Failed to insert session");

    // Query it back
    let mut stmt = db
        .prepare("SELECT id, household_id FROM sessions WHERE id = ?1")
        .expect("Failed to prepare statement");
    let result: (String, String) = stmt
        .query_row(params!["sess-001"], |row| Ok((row.get(0)?, row.get(1)?)))
        .expect("Failed to query session");

    assert_eq!(result.0, "sess-001");
    assert_eq!(result.1, "hh-001");
}

#[test]
fn test_transactions_table() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Insert a transaction
    db.execute(
        "INSERT INTO transactions (id, household_id, date, amount, merchant_name, import_fingerprint) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "txn-001",
            "hh-001",
            "2026-04-11",
            -25.50,
            "Starbucks",
            "fingerprint-123"
        ],
    )
    .expect("Failed to insert transaction");

    // Query it back
    let mut stmt = db
        .prepare("SELECT id, household_id, amount, merchant_name FROM transactions WHERE id = ?1")
        .expect("Failed to prepare statement");
    let result: (String, String, f64, String) = stmt
        .query_row(params!["txn-001"], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .expect("Failed to query transaction");

    assert_eq!(result.0, "txn-001");
    assert_eq!(result.1, "hh-001");
    assert_eq!(result.2, -25.50);
    assert_eq!(result.3, "Starbucks");
}

#[test]
fn test_foreign_key_constraint() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Try to insert a transaction without a household - should fail
    let result = db.execute(
        "INSERT INTO transactions (id, household_id, date, amount, merchant_name, import_fingerprint) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            "txn-002",
            "nonexistent-household",
            "2026-04-11",
            -50.00,
            "Amazon",
            "fingerprint-456"
        ],
    );

    assert!(result.is_err(), "Foreign key constraint should prevent invalid insert");
}

#[test]
fn test_transaction_categories_table() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create a category
    db.execute(
        "INSERT INTO transaction_categories (id, household_id, name, color, icon) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["cat-001", "hh-001", "Food", "#FF5733", "🍔"],
    )
    .expect("Failed to insert category");

    // Query it back
    let mut stmt = db
        .prepare("SELECT id, name, color FROM transaction_categories WHERE id = ?1")
        .expect("Failed to prepare statement");
    let result: (String, String, String) = stmt
        .query_row(params!["cat-001"], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .expect("Failed to query category");

    assert_eq!(result.0, "cat-001");
    assert_eq!(result.1, "Food");
    assert_eq!(result.2, "#FF5733");
}

#[test]
fn test_goals_table() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create a goal
    db.execute(
        "INSERT INTO goals (id, household_id, name, description, target_amount, deadline) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params!["goal-001", "hh-001", "Save for vacation", "Summer trip", 5000.0, "2026-08-31"],
    )
    .expect("Failed to insert goal");

    // Query it back
    let mut stmt = db
        .prepare("SELECT id, name, target_amount FROM goals WHERE id = ?1")
        .expect("Failed to prepare statement");
    let result: (String, String, f64) = stmt
        .query_row(params!["goal-001"], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .expect("Failed to query goal");

    assert_eq!(result.0, "goal-001");
    assert_eq!(result.1, "Save for vacation");
    assert_eq!(result.2, 5000.0);
}

// Query function tests
#[test]
fn test_create_and_list_transactions() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create transaction using query function
    let req = CreateTransactionRequest {
        date: "2026-04-11".to_string(),
        amount: -35.50,
        merchant_name: "Whole Foods".to_string(),
        category_id: None,
        description: Some("Groceries".to_string()),
        import_fingerprint: "fp-001".to_string(),
    };

    let txn = queries::create_transaction(&db, "hh-001", &req)
        .expect("Failed to create transaction");

    assert_eq!(txn.merchant_name, "Whole Foods");
    assert_eq!(txn.amount, -35.50);
    assert_eq!(txn.category_source, "uncategorized");

    // List transactions
    let txns = queries::list_transactions(&db, "hh-001", 10, 0)
        .expect("Failed to list transactions");

    assert_eq!(txns.len(), 1);
    assert_eq!(txns[0].id, txn.id);
}

#[test]
fn test_create_and_list_categories() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create category
    let req = CreateCategoryRequest {
        name: "Groceries".to_string(),
        color: Some("#2ecc71".to_string()),
        icon: Some("🛒".to_string()),
    };

    let cat = queries::create_category(&db, "hh-001", &req)
        .expect("Failed to create category");

    assert_eq!(cat.name, "Groceries");
    assert_eq!(cat.color, Some("#2ecc71".to_string()));

    // List categories
    let cats = queries::list_categories(&db, "hh-001")
        .expect("Failed to list categories");

    assert_eq!(cats.len(), 1);
    assert_eq!(cats[0].name, "Groceries");
}

#[test]
fn test_create_and_list_goals() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create goal
    let req = CreateGoalRequest {
        name: "Emergency Fund".to_string(),
        description: Some("6 months of expenses".to_string()),
        target_amount: 25000.0,
        deadline: Some("2027-12-31".to_string()),
        generates_income: None,
        creates_expenses: None,
    };

    let goal = queries::create_goal(&db, "hh-001", &req)
        .expect("Failed to create goal");

    assert_eq!(goal.name, "Emergency Fund");
    assert_eq!(goal.target_amount, 25000.0);
    assert_eq!(goal.current_amount, 0.0);

    // List goals
    let goals = queries::list_goals(&db, "hh-001")
        .expect("Failed to list goals");

    assert_eq!(goals.len(), 1);
    assert_eq!(goals[0].name, "Emergency Fund");
}

#[test]
fn test_session_lifecycle() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create session
    let session = queries::create_session(&db, "hh-001", "2026-12-31 23:59:59")
        .expect("Failed to create session");

    assert_eq!(session.household_id, "hh-001");

    // Get session
    let fetched = queries::get_session(&db, &session.id)
        .expect("Failed to get session")
        .expect("Session not found");

    assert_eq!(fetched.id, session.id);
    assert_eq!(fetched.household_id, "hh-001");

    // Delete session
    queries::delete_session(&db, &session.id)
        .expect("Failed to delete session");

    let deleted = queries::get_session(&db, &session.id)
        .expect("Failed to get session after delete");

    assert!(deleted.is_none());
}

#[test]
fn test_update_transaction() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create category
    let cat_req = CreateCategoryRequest {
        name: "Food".to_string(),
        color: None,
        icon: None,
    };
    let cat = queries::create_category(&db, "hh-001", &cat_req)
        .expect("Failed to create category");

    // Create transaction
    let txn_req = CreateTransactionRequest {
        date: "2026-04-11".to_string(),
        amount: -45.00,
        merchant_name: "Restaurant".to_string(),
        category_id: None,
        description: None,
        import_fingerprint: "fp-002".to_string(),
    };

    let txn = queries::create_transaction(&db, "hh-001", &txn_req)
        .expect("Failed to create transaction");

    // Update transaction with category
    let update_req = UpdateTransactionRequest {
        category_id: Some(cat.id.clone()),
        description: Some("Dinner out".to_string()),
        one_time_flag: Some(true),
    };

    queries::update_transaction(&db, &txn.id, &update_req)
        .expect("Failed to update transaction");

    // Verify update
    let updated = queries::get_transaction(&db, &txn.id)
        .expect("Failed to get transaction")
        .expect("Transaction not found");

    assert_eq!(updated.category_id, Some(cat.id));
    assert_eq!(updated.description, Some("Dinner out".to_string()));
    assert_eq!(updated.one_time_flag, true);
}

#[test]
fn test_create_and_list_goals_with_queries() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create multiple goals
    let goal1_req = CreateGoalRequest {
        name: "Emergency Fund".to_string(),
        description: Some("6 months of expenses".to_string()),
        target_amount: 25000.0,
        deadline: Some("2027-12-31".to_string()),
        generates_income: None,
        creates_expenses: None,
    };

    let goal1 = queries::create_goal(&db, "hh-001", &goal1_req)
        .expect("Failed to create goal 1");

    let goal2_req = CreateGoalRequest {
        name: "Vacation".to_string(),
        description: Some("Summer trip".to_string()),
        target_amount: 5000.0,
        deadline: Some("2026-08-31".to_string()),
        generates_income: Some(false),
        creates_expenses: Some(false),
    };

    let goal2 = queries::create_goal(&db, "hh-001", &goal2_req)
        .expect("Failed to create goal 2");

    // List goals
    let goals = queries::list_goals(&db, "hh-001")
        .expect("Failed to list goals");

    assert_eq!(goals.len(), 2);
    // Goals should be ordered by deadline
    assert_eq!(goals[0].name, "Vacation");
    assert_eq!(goals[1].name, "Emergency Fund");
}

#[test]
fn test_get_goal() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create goal
    let req = CreateGoalRequest {
        name: "Home Renovation".to_string(),
        description: Some("Kitchen and bathroom".to_string()),
        target_amount: 50000.0,
        deadline: Some("2027-06-30".to_string()),
        generates_income: None,
        creates_expenses: None,
    };

    let goal = queries::create_goal(&db, "hh-001", &req)
        .expect("Failed to create goal");

    // Get goal
    let fetched = queries::get_goal(&db, &goal.id)
        .expect("Failed to get goal")
        .expect("Goal not found");

    assert_eq!(fetched.id, goal.id);
    assert_eq!(fetched.name, "Home Renovation");
    assert_eq!(fetched.target_amount, 50000.0);
}

#[test]
fn test_update_goal() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create goal
    let req = CreateGoalRequest {
        name: "Car Fund".to_string(),
        description: None,
        target_amount: 30000.0,
        deadline: None,
        generates_income: Some(false),
        creates_expenses: Some(false),
    };

    let goal = queries::create_goal(&db, "hh-001", &req)
        .expect("Failed to create goal");

    // Update goal
    let update_req = UpdateGoalRequest {
        name: Some("Car Upgrade Fund".to_string()),
        description: Some("New car purchase".to_string()),
        target_amount: Some(40000.0),
        current_amount: Some(5000.0),
        deadline: Some("2027-03-31".to_string()),
        generates_income: Some(false),
        creates_expenses: None,
    };

    queries::update_goal(&db, &goal.id, &update_req)
        .expect("Failed to update goal");

    // Verify update
    let updated = queries::get_goal(&db, &goal.id)
        .expect("Failed to get goal")
        .expect("Goal not found");

    assert_eq!(updated.name, "Car Upgrade Fund");
    assert_eq!(updated.description, Some("New car purchase".to_string()));
    assert_eq!(updated.target_amount, 40000.0);
    assert_eq!(updated.current_amount, 5000.0);
    assert_eq!(updated.deadline, Some("2027-03-31".to_string()));
}

#[test]
fn test_delete_goal() {
    let db = init_test_db().expect("Failed to initialize test database");

    // Create household
    db.execute(
        "INSERT INTO households (id, name, password_hash) VALUES (?1, ?2, ?3)",
        params!["hh-001", "My Household", "hash123"],
    )
    .expect("Failed to insert household");

    // Create goal
    let req = CreateGoalRequest {
        name: "Temporary Goal".to_string(),
        description: None,
        target_amount: 1000.0,
        deadline: None,
        generates_income: None,
        creates_expenses: None,
    };

    let goal = queries::create_goal(&db, "hh-001", &req)
        .expect("Failed to create goal");

    // Verify goal exists
    let fetched = queries::get_goal(&db, &goal.id)
        .expect("Failed to get goal")
        .expect("Goal not found");
    assert_eq!(fetched.name, "Temporary Goal");

    // Delete goal
    queries::delete_goal(&db, &goal.id)
        .expect("Failed to delete goal");

    // Verify goal is deleted
    let deleted = queries::get_goal(&db, &goal.id)
        .expect("Failed to get goal after delete");

    assert!(deleted.is_none());
}
