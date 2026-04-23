use anyhow::Result;
use rusqlite::params;

use crate::db::Database;
use crate::index::IndexService;
use crate::model::IndexStatus;

#[derive(Clone)]
pub struct Scheduler {
  db: std::sync::Arc<Database>,
}

impl Scheduler {
  pub fn new(db: std::sync::Arc<Database>) -> Self {
    Self { db }
  }

  pub fn enqueue(&self, task_type: &str, payload: serde_json::Value) -> Result<()> {
    let conn = self
      .db
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute(
      "INSERT INTO tasks (task_type, payload_json, status) VALUES (?1, ?2, ?3)",
      params![task_type, payload.to_string(), "pending"],
    )?;
    Ok(())
  }

  pub fn index_status(&self) -> Result<IndexStatus> {
    let conn = self
      .db
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    let pending_tasks: usize = conn.query_row(
      "SELECT COUNT(*) FROM tasks WHERE status IN ('pending', 'running')",
      [],
      |row| row.get(0),
    )?;
    let indexed_files: usize = conn.query_row(
      "SELECT COUNT(*) FROM files WHERE index_status = 'indexed'",
      [],
      |row| row.get(0),
    )?;

    Ok(IndexStatus {
      pending_tasks,
      indexed_files,
    })
  }

  pub fn pop_next_task(&self) -> Result<Option<(i64, String, String)>> {
    let conn = self
      .db
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    let mut stmt = conn.prepare(
      "SELECT id, task_type, payload_json FROM tasks WHERE status = 'pending' ORDER BY priority ASC, id ASC LIMIT 1",
    )?;
    let row = stmt.query_row([], |row| {
      Ok((
        row.get::<_, i64>(0)?,
        row.get::<_, String>(1)?,
        row.get::<_, String>(2)?,
      ))
    });
    let task = match row {
      Ok(value) => value,
      Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
      Err(err) => return Err(err.into()),
    };
    conn.execute(
      "UPDATE tasks SET status = 'running', started_at = CURRENT_TIMESTAMP WHERE id = ?1",
      params![task.0],
    )?;
    Ok(Some(task))
  }

  pub fn complete_task(&self, task_id: i64) -> Result<()> {
    let conn = self
      .db
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    conn.execute(
      "UPDATE tasks SET status = 'done', finished_at = CURRENT_TIMESTAMP WHERE id = ?1",
      params![task_id],
    )?;
    Ok(())
  }

  pub fn run_index_batch(&self, index_service: &IndexService, limit: usize) -> Result<usize> {
    let files = self.db.list_pending_index_files(limit)?;
    let mut processed = 0usize;
    for (file_id, path, filename) in files {
      let content = std::fs::read_to_string(&path).unwrap_or_default();
      index_service.index_text_document(file_id, &path, &filename, &content)?;
      self.db.mark_indexed(file_id)?;
      processed += 1;
    }
    Ok(processed)
  }
}
