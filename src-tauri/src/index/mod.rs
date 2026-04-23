use std::path::PathBuf;

use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::{AllQuery, Query, QueryParser};
use tantivy::schema::{SchemaBuilder, Value, FAST, STORED, STRING, TEXT};
use tantivy::{doc, Index};

#[derive(Clone)]
pub struct IndexService {
  index: Index,
}

impl IndexService {
  pub fn open_default() -> Result<Self> {
    let index_path = PathBuf::from(".data/index");
    std::fs::create_dir_all(&index_path)?;

    let mut schema_builder = SchemaBuilder::default();
    let _file_id = schema_builder.add_u64_field("file_id", FAST | STORED);
    let _path = schema_builder.add_text_field("path", TEXT | STORED);
    let _filename = schema_builder.add_text_field("filename", TEXT | STORED);
    let _extension = schema_builder.add_text_field("extension", STRING | STORED);
    let _mime = schema_builder.add_text_field("mime", STRING | STORED);
    let _content = schema_builder.add_text_field("content", TEXT);
    let _title = schema_builder.add_text_field("title", TEXT | STORED);
    let _size_bytes = schema_builder.add_u64_field("size_bytes", FAST | STORED);
    let schema = schema_builder.build();

    let index = match Index::open_in_dir(&index_path) {
      Ok(existing) => existing,
      Err(_) => Index::create_in_dir(&index_path, schema)?,
    };

    Ok(Self { index })
  }

  pub fn index(&self) -> &Index {
    &self.index
  }

  pub fn index_text_document(
    &self,
    file_id: i64,
    path: &str,
    filename: &str,
    content: &str,
  ) -> Result<()> {
    let schema = self.index.schema();
    let file_id_field = schema.get_field("file_id")?;
    let path_field = schema.get_field("path")?;
    let filename_field = schema.get_field("filename")?;
    let extension_field = schema.get_field("extension")?;
    let mime_field = schema.get_field("mime")?;
    let content_field = schema.get_field("content")?;
    let title_field = schema.get_field("title")?;
    let size_field = schema.get_field("size_bytes")?;

    let mut writer = self.index.writer(20_000_000)?;
    let extension = std::path::Path::new(path)
      .extension()
      .map(|v| v.to_string_lossy().to_string())
      .unwrap_or_default();
    writer.add_document(doc!(
      file_id_field => file_id as u64,
      path_field => path.to_string(),
      filename_field => filename.to_string(),
      extension_field => extension,
      mime_field => "text/plain".to_string(),
      content_field => content.to_string(),
      title_field => filename.to_string(),
      size_field => content.len() as u64,
    ))?;
    writer.commit()?;
    Ok(())
  }

  pub fn search(&self, query: &str, limit: usize) -> Result<Vec<(i64, String, String)>> {
    let reader = self.index.reader()?;
    let searcher = reader.searcher();
    let schema = self.index.schema();
    let content_field = schema.get_field("content")?;
    let filename_field = schema.get_field("filename")?;
    let path_field = schema.get_field("path")?;
    let file_id_field = schema.get_field("file_id")?;

    let parser = QueryParser::for_index(&self.index, vec![filename_field, content_field]);
    let trimmed = query.trim();
    let query_obj: Box<dyn Query> = if trimmed.is_empty() || trimmed == "*" {
      Box::new(AllQuery)
    } else {
      parser.parse_query(trimmed)?
    };
    let top_docs = searcher.search(&*query_obj, &TopDocs::with_limit(limit))?;
    let mut out = Vec::new();
    for (_, addr) in top_docs {
      let doc = searcher.doc::<tantivy::TantivyDocument>(addr)?;
      let file_id = doc
        .get_first(file_id_field)
        .and_then(|v| v.as_u64())
        .unwrap_or_default() as i64;
      let path = doc
        .get_first(path_field)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
      let filename = doc
        .get_first(filename_field)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
      out.push((file_id, path, filename));
    }
    Ok(out)
  }
}
