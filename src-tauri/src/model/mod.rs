use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Root {
  pub id: i64,
  pub path: String,
  pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
  pub indexed_roots: Vec<String>,
  pub ignored_globs: Vec<String>,
  pub max_file_size_mb: u32,
  pub watch_enabled: bool,
  pub result_page_size: u32,
  pub log_level: String,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      indexed_roots: vec![],
      ignored_globs: vec!["**/.git/**".to_string(), "**/node_modules/**".to_string()],
      max_file_size_mb: 20,
      watch_enabled: true,
      result_page_size: 50,
      log_level: "info".to_string(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
  pub query: String,
  pub page: u32,
  pub page_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
  pub file_id: i64,
  pub path: String,
  pub filename: String,
  pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
  pub hits: Vec<SearchHit>,
  pub total: usize,
  pub took_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
  pub pending_tasks: usize,
  pub indexed_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCandidate {
  pub root_id: i64,
  pub path: String,
  pub parent_path: String,
  pub filename: String,
  pub stem: String,
  pub extension: String,
  pub size_bytes: u64,
  pub modified_at_fs: Option<String>,
  pub fingerprint: String,
}
