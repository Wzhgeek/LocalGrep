pub struct ExtractInput {
  pub path: String,
}

pub struct ExtractOutput {
  pub title: String,
  pub plain_text: String,
}

pub trait Extractor: Send + Sync {
  fn id(&self) -> &'static str;
  fn supports(&self, ext: &str, mime: &str) -> bool;
  fn extract(&self, input: &ExtractInput) -> anyhow::Result<ExtractOutput>;
}
