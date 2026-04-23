use anyhow::Result;
use walkdir::WalkDir;

use crate::db::Database;
use crate::model::FileCandidate;
use crate::scheduler::Scheduler;

#[derive(Clone)]
pub struct ScanService {
  _db: std::sync::Arc<Database>,
  scheduler: std::sync::Arc<Scheduler>,
}

impl ScanService {
  pub fn new(db: std::sync::Arc<Database>, scheduler: std::sync::Arc<Scheduler>) -> Self {
    Self { _db: db, scheduler }
  }

  pub async fn start_full_scan(&self) -> Result<()> {
    let conn = self
      ._db
      .conn
      .lock()
      .map_err(|_| anyhow::anyhow!("db lock poisoned"))?;
    let mut stmt = conn.prepare("SELECT id, path FROM roots WHERE enabled = 1 ORDER BY id ASC")?;
    let rows = stmt.query_map([], |row| {
      Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    let mut roots: Vec<(i64, String)> = Vec::new();
    for row in rows {
      roots.push(row?);
    }
    drop(stmt);
    drop(conn);

    for (root_id, root_path) in roots {
      let mut existing_paths = Vec::new();
      for entry in WalkDir::new(&root_path).follow_links(false) {
        let entry = match entry {
          Ok(v) => v,
          Err(_) => continue,
        };
        if entry.file_type().is_dir() {
          continue;
        }
        let path = entry.path().to_string_lossy().to_string();
        let parent_path = entry
          .path()
          .parent()
          .map(|v| v.to_string_lossy().to_string())
          .unwrap_or_default();
        let filename = entry.file_name().to_string_lossy().to_string();
        let stem = entry
          .path()
          .file_stem()
          .map(|v| v.to_string_lossy().to_string())
          .unwrap_or_default();
        let extension = entry
          .path()
          .extension()
          .map(|v| v.to_string_lossy().to_string())
          .unwrap_or_default();
        let metadata = match entry.metadata() {
          Ok(v) => v,
          Err(_) => continue,
        };
        let modified_at_fs = metadata
          .modified()
          .ok()
          .and_then(|v| v.duration_since(std::time::UNIX_EPOCH).ok())
          .map(|v| v.as_secs().to_string());
        let fingerprint = format!(
          "{}:{}:{}",
          path,
          metadata.len(),
          modified_at_fs.clone().unwrap_or_default()
        );

        let candidate = FileCandidate {
          root_id,
          path: path.clone(),
          parent_path,
          filename,
          stem,
          extension,
          size_bytes: metadata.len(),
          modified_at_fs,
          fingerprint,
        };
        self._db.upsert_file_candidate(&candidate)?;
        self
          .scheduler
          .enqueue("index_file", serde_json::json!({ "path": path }))?;
        existing_paths.push(candidate.path);
      }
      self
        ._db
        .mark_missing_files_by_root(root_id, &existing_paths)?;
    }

    Ok(())
  }
}
