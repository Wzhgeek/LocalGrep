PRAGMA journal_mode = WAL;

CREATE TABLE IF NOT EXISTS roots (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  path TEXT NOT NULL UNIQUE,
  enabled INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS files (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  root_id INTEGER NOT NULL,
  path TEXT NOT NULL UNIQUE,
  parent_path TEXT NOT NULL,
  filename TEXT NOT NULL,
  stem TEXT NOT NULL,
  extension TEXT NOT NULL,
  mime TEXT NOT NULL,
  size_bytes INTEGER NOT NULL,
  created_at_fs TEXT,
  modified_at_fs TEXT,
  fingerprint TEXT,
  is_hidden INTEGER NOT NULL DEFAULT 0,
  is_symlink INTEGER NOT NULL DEFAULT 0,
  exists_flag INTEGER NOT NULL DEFAULT 1,
  extract_status TEXT NOT NULL DEFAULT 'pending',
  index_status TEXT NOT NULL DEFAULT 'pending',
  last_indexed_at TEXT,
  last_error_code TEXT,
  last_error_message TEXT
);

CREATE INDEX IF NOT EXISTS idx_files_fingerprint ON files (fingerprint);
CREATE INDEX IF NOT EXISTS idx_files_root_modified ON files (root_id, modified_at_fs);
CREATE INDEX IF NOT EXISTS idx_files_extract_index ON files (extract_status, index_status);

CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  task_type TEXT NOT NULL,
  file_id INTEGER,
  payload_json TEXT NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',
  priority INTEGER NOT NULL DEFAULT 50,
  retry_count INTEGER NOT NULL DEFAULT 0,
  scheduled_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  started_at TEXT,
  finished_at TEXT,
  error_message TEXT
);

CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS search_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  query TEXT NOT NULL,
  filters_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS diagnostics (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  category TEXT NOT NULL,
  level TEXT NOT NULL,
  message TEXT NOT NULL,
  payload_json TEXT NOT NULL DEFAULT '{}',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
