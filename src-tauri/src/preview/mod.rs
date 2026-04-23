pub fn build_preview(content: &str, limit: usize) -> String {
  content.chars().take(limit).collect()
}
