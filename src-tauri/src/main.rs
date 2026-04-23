#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod command;
mod config;
mod db;
mod diagnostics;
mod extractor;
mod index;
mod model;
mod normalizer;
mod preview;
mod query;
mod scanner;
mod scheduler;
mod state;
mod util;
mod watcher;

use crate::app::build_app;

fn main() {
  build_app();
}
