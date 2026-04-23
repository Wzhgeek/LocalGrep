use crate::state::AppState;
use crate::util::logging::init_logging;

pub fn build_app() {
  init_logging();
  let state = AppState::bootstrap().expect("failed to bootstrap app state");

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![
      crate::command::get_settings,
      crate::command::update_settings,
      crate::command::list_roots,
      crate::command::add_root,
      crate::command::remove_root,
      crate::command::start_full_scan,
      crate::command::get_index_status,
      crate::command::search,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
