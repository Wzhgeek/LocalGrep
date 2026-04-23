#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 骨架 API 与模块在功能接满前可能未使用；CI 使用 -D warnings 时需保留本行。
#![allow(dead_code)]

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
