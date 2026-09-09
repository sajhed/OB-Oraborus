mod agents;
mod api_registry;
mod audit;
mod automation;
mod browser;
mod commands;
mod data_analysis;
mod db;
mod documents;
mod emotional_context;
mod error;
mod files;
mod github;
mod memory;
mod model_router;
mod models;
mod plugins;
mod providers;
mod research;
mod secrets;
mod security;
mod state;
mod system_monitor;
mod terminal;
mod tool_registry;
mod tray;
mod vision;
mod voice;
mod windows_control;

use tauri::Manager;

pub fn run(){
 tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).json().with_target(false).init();
 tauri::Builder::default()
  .plugin(tauri_plugin_single_instance::init(|app,_,_|{if let Some(window)=app.get_webview_window("main"){let _=window.show();let _=window.unminimize();let _=window.set_focus();}}))
  .plugin(tauri_plugin_global_shortcut::Builder::new().build())
  .plugin(tauri_plugin_notification::init())
  .plugin(tauri_plugin_autostart::Builder::new().args(["--minimized"]).build())
  .setup(|app|{
   let data_dir=app.path().app_data_dir()?;
   let resource_dir=app.path().resource_dir()?;
   let state=state::AppState::initialize(data_dir,resource_dir).map_err(|error|std::io::Error::other(error.to_string()))?;
   app.manage(state);
   tray::install(app)?;
   Ok(())
  })
  .invoke_handler(tauri::generate_handler![
   commands::get_app_snapshot,
   commands::run_command,
   commands::stream_chat,
   commands::resolve_approval,
   commands::stop_all,
   commands::set_paused,
   commands::get_system_snapshot,
   commands::get_provider_health,
   commands::store_provider_secret,
   commands::set_privacy_capability,
   commands::list_voice_engines,
   commands::get_api_registry,
   commands::extract_document
  ])
  .run(tauri::generate_context!())
  .expect("OB Oraborus application failed to start");
}
