use std::time::Instant;

use anyhow::Result;

use crate::index::IndexService;
use crate::model::{SearchRequest, SearchResponse};

#[derive(Clone)]
pub struct QueryService {
  index_service: std::sync::Arc<IndexService>,
}

impl QueryService {
  pub fn new(index_service: std::sync::Arc<IndexService>) -> Self {
    Self { index_service }
  }

  pub async fn search(&self, input: SearchRequest) -> Result<SearchResponse> {
    let started = Instant::now();
    let query = if input.query.trim().is_empty() {
      "*".to_string()
    } else {
      input.query
    };
    let rows = self
      .index_service
      .search(&query, input.page_size.max(1) as usize)?;
    let hits = rows
      .into_iter()
      .map(|(file_id, path, filename)| crate::model::SearchHit {
        file_id,
        path,
        filename,
        snippet: "命中内容预览将在 Preview Service 阶段增强".to_string(),
      })
      .collect::<Vec<_>>();

    Ok(SearchResponse {
      total: hits.len(),
      hits,
      took_ms: started.elapsed().as_millis() as u64,
    })
  }
}
