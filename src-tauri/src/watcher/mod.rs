#[derive(Debug, Clone)]
pub enum WatchEvent {
  Created(String),
  Modified(String),
  Deleted(String),
  Renamed { from: String, to: String },
}

pub fn normalize_event(kind: &str, path: &str) -> Option<WatchEvent> {
  match kind {
    "create" => Some(WatchEvent::Created(path.to_string())),
    "modify" => Some(WatchEvent::Modified(path.to_string())),
    "remove" => Some(WatchEvent::Deleted(path.to_string())),
    _ => None,
  }
}
