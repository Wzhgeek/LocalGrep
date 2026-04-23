#[cfg(test)]
mod tests {
  use crate::normalizer::normalize_text;

  #[test]
  fn normalizer_collapses_whitespace() {
    assert_eq!(normalize_text("a   b\n\t c"), "a b c");
  }
}
