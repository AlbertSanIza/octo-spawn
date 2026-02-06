use std::process::Command;
use tauri::{Manager, PhysicalPosition, TrayIconEvent};

// Execute a shell command
#[tauri::command]
fn execute_command(command: String) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to execute command: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![execute_command])
        .setup(|app| {
            // Handle tray icon clicks
            let window = app.get_webview_window("main").unwrap();
            
            app.tray_by_id("tray").unwrap().on_tray_icon_event(move |_tray, event| {
                if let TrayIconEvent::Click { button, .. } = event {
                    if button == tauri::tray::MouseButton::Left {
                        let window = window.clone();
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            // Position window near cursor (menubar area)
                            if let Ok(cursor_pos) = window.cursor_position() {
                                let _ = window.set_position(PhysicalPosition::new(
                                    cursor_pos.x as i32 - 150,
                                    30,
                                ));
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
