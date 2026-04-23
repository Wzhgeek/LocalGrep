use crate::model::{IndexStatus, Root, SearchRequest, SearchResponse, Settings};
use crate::state::AppState;

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, AppState>) -> Result<Settings, String> {
  state.config_service().get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_settings(state: tauri::State<'_, AppState>, input: Settings) -> Result<(), String> {
  state
    .config_service()
    .update_settings(input)
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_roots(state: tauri::State<'_, AppState>) -> Result<Vec<Root>, String> {
  state.config_service().list_roots().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_root(state: tauri::State<'_, AppState>, path: String) -> Result<(), String> {
  state
    .config_service()
    .add_root(&path)
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_root(state: tauri::State<'_, AppState>, root_id: i64) -> Result<(), String> {
  state
    .config_service()
    .remove_root(root_id)
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_full_scan(state: tauri::State<'_, AppState>) -> Result<(), String> {
  state.scanner().start_full_scan().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_index_status(state: tauri::State<'_, AppState>) -> Result<IndexStatus, String> {
  state.scheduler().index_status().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search(
  state: tauri::State<'_, AppState>,
  input: SearchRequest,
) -> Result<SearchResponse, String> {
  state.query_service().search(input).await.map_err(|e| e.to_string())
}
