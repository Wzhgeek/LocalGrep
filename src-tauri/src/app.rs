use crate::command::{
  add_root, get_index_status, get_settings, list_roots, remove_root, search, start_full_scan,
  update_settings,
};
use crate::state::AppState;
use crate::util::logging::init_logging;

pub fn build_app() {
  init_logging();
  let state = AppState::bootstrap().expect("failed to bootstrap app state");

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      get_settings,
      update_settings,
      list_roots,
      add_root,
      remove_root,
      start_full_scan,
      get_index_status,
      search,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
