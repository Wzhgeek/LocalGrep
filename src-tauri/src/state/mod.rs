use std::sync::Arc;

use anyhow::Result;

use crate::config::ConfigService;
use crate::db::Database;
use crate::index::IndexService;
use crate::query::QueryService;
use crate::scanner::ScanService;
use crate::scheduler::Scheduler;

#[derive(Clone)]
pub struct AppState {
  config_service: Arc<ConfigService>,
  scanner: Arc<ScanService>,
  scheduler: Arc<Scheduler>,
  index_service: Arc<IndexService>,
  query_service: Arc<QueryService>,
}

impl AppState {
  pub fn bootstrap() -> Result<Self> {
    let database = Arc::new(Database::connect()?);
    database.run_migrations()?;

    let index_service = Arc::new(IndexService::open_default()?);
    let config_service = Arc::new(ConfigService::new(database.clone()));
    let scheduler = Arc::new(Scheduler::new(database.clone()));
    let scanner = Arc::new(ScanService::new(database.clone(), scheduler.clone()));
    let query_service = Arc::new(QueryService::new(index_service.clone()));

    let scheduler_bg = scheduler.clone();
    let index_bg = index_service.clone();
    tauri::async_runtime::spawn(async move {
      loop {
        let _ = scheduler_bg.run_index_batch(&index_bg, 32);
        tokio::time::sleep(std::time::Duration::from_millis(800)).await;
      }
    });

    Ok(Self {
      config_service,
      scanner,
      scheduler,
      index_service,
      query_service,
    })
  }

  pub fn config_service(&self) -> Arc<ConfigService> {
    self.config_service.clone()
  }

  pub fn scanner(&self) -> Arc<ScanService> {
    self.scanner.clone()
  }

  pub fn scheduler(&self) -> Arc<Scheduler> {
    self.scheduler.clone()
  }

  pub fn query_service(&self) -> Arc<QueryService> {
    self.query_service.clone()
  }

  pub fn index_service(&self) -> Arc<IndexService> {
    self.index_service.clone()
  }
}
