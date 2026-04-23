pub fn normalize_text(raw: &str) -> String {
  raw.split_whitespace().collect::<Vec<_>>().join(" ")
}
