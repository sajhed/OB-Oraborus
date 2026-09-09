use crate::state::AppState;
use tauri::{menu::{Menu,MenuItem},tray::TrayIconBuilder,Emitter,Manager};

pub fn install(app:&tauri::App)->tauri::Result<()> {
 let open=MenuItem::with_id(app,"open","Open OB",true,None::<&str>)?;
 let quick=MenuItem::with_id(app,"quick","Quick Command",true,None::<&str>)?;
 let pause=MenuItem::with_id(app,"pause","Pause",true,None::<&str>)?;
 let resume=MenuItem::with_id(app,"resume","Resume",true,None::<&str>)?;
 let stop=MenuItem::with_id(app,"stop","Stop All",true,None::<&str>)?;
 let privacy=MenuItem::with_id(app,"privacy","Privacy Mode",true,None::<&str>)?;
 let settings=MenuItem::with_id(app,"settings","Settings",true,None::<&str>)?;
 let exit=MenuItem::with_id(app,"exit","Exit",true,None::<&str>)?;
 let menu=Menu::with_items(app,&[&open,&quick,&pause,&resume,&stop,&privacy,&settings,&exit])?;
 TrayIconBuilder::with_id("ob-main-tray").menu(&menu).show_menu_on_left_click(false).on_menu_event(|app,event|match event.id().as_ref(){
  "open"=>show(app),
  "quick"=>{show(app);let _=app.emit("ob://quick-command",());},
  "pause"=>{let state=app.state::<AppState>();let _=state.db.set_setting("paused",&true);},
  "resume"=>{let state=app.state::<AppState>();let _=state.db.set_setting("paused",&false);},
  "stop"=>{let state=app.state::<AppState>();state.stop_all();let _=state.automations.disable_all();},
  "privacy"=>{let state=app.state::<AppState>();let _=state.db.set_setting("local_only",&true);let _=state.db.set_setting("privacy.microphone",&false);let _=state.db.set_setting("privacy.camera",&false);let _=state.db.set_setting("privacy.screen_capture",&false);},
  "settings"=>{show(app);let _=app.emit("ob://open-settings",());},
  "exit"=>app.exit(0),
  _=>{}
 }).build(app)?;
 Ok(())
}
fn show(app:&tauri::AppHandle){if let Some(window)=app.get_webview_window("main"){let _=window.show();let _=window.unminimize();let _=window.set_focus();}}
