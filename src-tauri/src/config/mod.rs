use anyhow::Result;

use crate::db::Database;
use crate::model::{Root, Settings};

#[derive(Clone)]
pub struct ConfigService {
  db: std::sync::Arc<Database>,
}

impl ConfigService {
  pub fn new(db: std::sync::Arc<Database>) -> Self {
    Self { db }
  }

  pub fn get_settings(&self) -> Result<Settings> {
    self.db.get_settings()
  }

  pub fn update_settings(&self, settings: Settings) -> Result<()> {
    self.db.save_settings(settings)
  }

  pub fn list_roots(&self) -> Result<Vec<Root>> {
    self.db.list_roots()
  }

  pub fn add_root(&self, path: &str) -> Result<()> {
    self.db.add_root(path)
  }

  pub fn remove_root(&self, root_id: i64) -> Result<()> {
    self.db.remove_root(root_id)
  }
}
