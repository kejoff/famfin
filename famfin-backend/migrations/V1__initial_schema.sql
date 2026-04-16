-- Initial schema for famfin

-- Sessions table (httpOnly cookie + server-side state)
CREATE TABLE sessions (
  id TEXT PRIMARY KEY,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at DATETIME NOT NULL,
  household_id TEXT NOT NULL
);

-- Households (each Pi instance manages one household)
CREATE TABLE households (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  password_hash TEXT NOT NULL,
  db_version INTEGER NOT NULL DEFAULT 1
);

-- Goals (savings objectives, timelines)
CREATE TABLE goals (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  name TEXT NOT NULL,
  description TEXT,
  target_amount REAL NOT NULL,
  current_amount REAL NOT NULL DEFAULT 0,
  deadline DATE,
  generates_income INTEGER NOT NULL DEFAULT 0,
  creates_expenses INTEGER NOT NULL DEFAULT 0,
  metadata TEXT,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- Transactions (imported from CSV/OFX)
CREATE TABLE transactions (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  date DATE NOT NULL,
  amount REAL NOT NULL,
  merchant_name TEXT NOT NULL,
  category_id TEXT REFERENCES transaction_categories(id),
  description TEXT,
  category_source TEXT NOT NULL DEFAULT 'uncategorized',
  one_time_flag INTEGER NOT NULL DEFAULT 0,
  import_fingerprint TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- Transaction categories (learned from user corrections)
CREATE TABLE transaction_categories (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  name TEXT NOT NULL,
  color TEXT,
  icon TEXT,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- Category rules (merchant → category mapping learned from corrections)
CREATE TABLE category_rules (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  merchant_pattern TEXT NOT NULL,
  category_id TEXT NOT NULL REFERENCES transaction_categories(id),
  confidence REAL NOT NULL DEFAULT 1.0,
  source TEXT NOT NULL DEFAULT 'user',
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- Receipts (from photo scanning)
CREATE TABLE receipts (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  photo_path TEXT NOT NULL,
  extracted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  pending_match INTEGER NOT NULL DEFAULT 1,
  matched_transaction_id TEXT REFERENCES transactions(id),
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- Receipt items (line items from receipt OCR)
CREATE TABLE receipt_items (
  id TEXT PRIMARY KEY,
  receipt_id TEXT NOT NULL REFERENCES receipts(id),
  description TEXT NOT NULL,
  amount REAL NOT NULL,
  quantity REAL,
  category_id TEXT REFERENCES transaction_categories(id),
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(receipt_id) REFERENCES receipts(id)
);

-- Earmarked resources (Ticket Restaurant, etc.)
CREATE TABLE earmarked_resources (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  name TEXT NOT NULL,
  resource_type TEXT NOT NULL,
  balance REAL NOT NULL,
  provider TEXT,
  expires_at DATE,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- LLM API keys (stored encrypted in SQLCipher)
CREATE TABLE llm_config (
  id TEXT PRIMARY KEY,
  household_id TEXT NOT NULL REFERENCES households(id),
  provider TEXT NOT NULL,
  api_key TEXT NOT NULL,
  endpoint TEXT,
  model TEXT,
  enabled INTEGER NOT NULL DEFAULT 1,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(household_id) REFERENCES households(id)
);

-- Indices for performance
CREATE INDEX idx_transactions_household_id ON transactions(household_id);
CREATE INDEX idx_transactions_date ON transactions(date);
CREATE INDEX idx_transactions_import_fingerprint ON transactions(import_fingerprint);
CREATE INDEX idx_goals_household_id ON goals(household_id);
CREATE INDEX idx_category_rules_household_id ON category_rules(household_id);
CREATE INDEX idx_receipts_household_id ON receipts(household_id);
CREATE INDEX idx_receipts_pending_match ON receipts(pending_match);
CREATE INDEX idx_sessions_household_id ON sessions(household_id);
