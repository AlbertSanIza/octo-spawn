use std::process::Command;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};

fn spawn_github_desktop() -> Result<(), String> {
    Command::new("open")
        .args(["-na", "GitHub Desktop"])
        .spawn()
        .map_err(|e| format!("Failed to spawn GitHub Desktop: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let spawn_item =
                MenuItemBuilder::with_id("spawn", "Spawn Github Desktop").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&spawn_item)
                .separator()
                .item(&quit_item)
                .build()?;
            let _tray = TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "spawn" => {
                        if let Err(error) = spawn_github_desktop() {
                            eprintln!("Error: {}", error);
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
