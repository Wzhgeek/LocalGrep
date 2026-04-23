#[derive(Debug, Clone)]
pub enum WatchEvent {
  FileCreated(String),
  FileModified(String),
  FileDeleted(String),
  FileRenamed { from: String, to: String },
}

pub fn normalize_event(kind: &str, path: &str) -> Option<WatchEvent> {
  match kind {
    "create" => Some(WatchEvent::FileCreated(path.to_string())),
    "modify" => Some(WatchEvent::FileModified(path.to_string())),
    "remove" => Some(WatchEvent::FileDeleted(path.to_string())),
    _ => None,
  }
}
