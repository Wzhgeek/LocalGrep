use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::{params, Connection};

use crate::model::{FileCandidate, Root, Settings};

#[derive(Debug)]
pub struct Database {
  pub(crate) conn: std::sync::Mutex<Connection>,
}

impl Database {
  pub fn connect() -> Result<Self> {
    let db_dir = PathBuf::from(".data");
    fs::create_dir_all(&db_dir)?;
    let db_path = db_dir.join("localgrep.db");
    let conn = Connection::open(db_path)?;
    Ok(Self {
      conn: std::sync::Mutex::new(conn),
    })
  }

  pub fn run_migrations(&self) -> Result<()> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute_batch(include_str!("../sql/001_init.sql"))?;
    Ok(())
  }

  pub fn get_settings(&self) -> Result<Settings> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    let mut stmt = conn.prepare("SELECT value_json FROM settings WHERE key = ?1")?;
    let payload = stmt
      .query_row(["app_settings"], |row| row.get::<_, String>(0))
      .unwrap_or_else(|_| {
        serde_json::to_string(&Settings::default()).unwrap_or_else(|_| "{}".to_string())
      });

    serde_json::from_str(&payload).context("parse settings payload failed")
  }

  pub fn save_settings(&self, settings: Settings) -> Result<()> {
    let payload = serde_json::to_string(&settings)?;
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute(
      "INSERT INTO settings (key, value_json) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json",
      params!["app_settings", payload],
    )?;
    Ok(())
  }

  pub fn list_roots(&self) -> Result<Vec<Root>> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    let mut stmt = conn.prepare("SELECT id, path, enabled FROM roots ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| {
      Ok(Root {
        id: row.get(0)?,
        path: row.get(1)?,
        enabled: row.get(2)?,
      })
    })?;

    let mut roots = Vec::new();
    for row in rows {
      roots.push(row?);
    }
    Ok(roots)
  }

  pub fn add_root(&self, path: &str) -> Result<()> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute(
      "INSERT OR IGNORE INTO roots (path, enabled) VALUES (?1, 1)",
      params![path],
    )?;
    Ok(())
  }

  pub fn remove_root(&self, root_id: i64) -> Result<()> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute("DELETE FROM roots WHERE id = ?1", params![root_id])?;
    Ok(())
  }

  pub fn upsert_file_candidate(&self, item: &FileCandidate) -> Result<()> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute(
      "INSERT INTO files (
        root_id, path, parent_path, filename, stem, extension, mime, size_bytes, modified_at_fs, fingerprint, index_status
      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 'pending')
      ON CONFLICT(path) DO UPDATE SET
        root_id=excluded.root_id,
        parent_path=excluded.parent_path,
        filename=excluded.filename,
        stem=excluded.stem,
        extension=excluded.extension,
        size_bytes=excluded.size_bytes,
        modified_at_fs=excluded.modified_at_fs,
        fingerprint=excluded.fingerprint,
        exists_flag=1,
        index_status='pending'",
      params![
        item.root_id,
        item.path,
        item.parent_path,
        item.filename,
        item.stem,
        item.extension,
        "application/octet-stream",
        item.size_bytes,
        item.modified_at_fs,
        item.fingerprint,
      ],
    )?;
    Ok(())
  }

  pub fn mark_missing_files_by_root(&self, root_id: i64, existing_paths: &[String]) -> Result<()> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    if existing_paths.is_empty() {
      conn.execute(
        "UPDATE files SET exists_flag = 0, index_status = 'pending' WHERE root_id = ?1",
        params![root_id],
      )?;
      return Ok(());
    }

    let placeholders = vec!["?"; existing_paths.len()].join(", ");
    let sql = format!(
      "UPDATE files SET exists_flag = 0, index_status = 'pending' WHERE root_id = ?1 AND path NOT IN ({})",
      placeholders
    );
    let mut values: Vec<rusqlite::types::Value> = Vec::with_capacity(existing_paths.len() + 1);
    values.push(root_id.into());
    for path in existing_paths {
      values.push(path.clone().into());
    }
    conn.execute(&sql, rusqlite::params_from_iter(values))?;
    Ok(())
  }

  pub fn list_pending_index_files(&self, limit: usize) -> Result<Vec<(i64, String, String)>> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    let mut stmt = conn.prepare(
      "SELECT id, path, filename FROM files WHERE exists_flag = 1 AND index_status = 'pending' ORDER BY id ASC LIMIT ?1",
    )?;
    let rows = stmt.query_map(params![limit as i64], |row| {
      Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;
    let mut out = Vec::new();
    for row in rows {
      out.push(row?);
    }
    Ok(out)
  }

  pub fn mark_indexed(&self, file_id: i64) -> Result<()> {
    let conn = self
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute(
      "UPDATE files SET index_status = 'indexed', last_indexed_at = CURRENT_TIMESTAMP WHERE id = ?1",
      params![file_id],
    )?;
    Ok(())
  }
}
