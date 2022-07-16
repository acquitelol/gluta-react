#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{SystemTray, SystemTrayEvent};
use tauri::{Manager, LogicalPosition, LogicalSize};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


fn main() {
  let tray = SystemTray::new();

  tauri::Builder::default()
    .system_tray(tray)
    .on_system_tray_event(move |app, event| match event {
      

      SystemTrayEvent::LeftClick {
        position: physical_position, size: _, ..
      } => {
        let window = app.get_window("main").unwrap();
        let a = window.is_visible().unwrap();
        let logical_size: LogicalSize<f64> = tauri::LogicalSize::<f64> {
          width: 250.00,
          height: 360.00,
        };
  
        let logical_position: LogicalPosition<f64> = tauri::LogicalPosition::<f64> {
          x: (physical_position.x - logical_size.width + 30.0)/2.0,
          y: 0.0,
        };
        let logical_pos: tauri::Position = tauri::Position::Logical(logical_position);
        window.set_position(logical_position).unwrap();
        
        if !a {
          window.set_focus().unwrap();
        }
      }
      SystemTrayEvent::RightClick {
        position: _,
        size: _,
        ..
      } => {
        
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      window.hide().unwrap();
      window.set_decorations(false).unwrap();

      Ok(())
    })
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::Focused(focused) => {
        if !focused {
          event.window().hide().unwrap();
        }
        // hide window whenever it loses focus
        
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
