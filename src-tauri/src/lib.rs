use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};

const CLONE_DIR: &str = "/tmp/github-desktop-clone";
const SINGLETON_FILES: &[&str] = &["SingletonLock", "SingletonCookie", "SingletonSocket"];

fn source_user_data_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join("Library/Application Support/GitHub Desktop")
}

fn prepare_clone_dir() -> Result<(), String> {
    let clone_path = Path::new(CLONE_DIR);
    if !clone_path.exists() {
        let source = source_user_data_dir();
        let status = Command::new("cp")
            .args(["-R", &source.to_string_lossy(), CLONE_DIR])
            .status()
            .map_err(|e| format!("Failed to copy user data: {}", e))?;
        if !status.success() {
            return Err("cp command failed".into());
        }
    }
    for file in SINGLETON_FILES {
        let _ = fs::remove_file(clone_path.join(file));
    }
    Ok(())
}

fn spawn_github_desktop() -> Result<(), String> {
    prepare_clone_dir()?;
    Command::new("open")
        .args([
            "-n",
            "-a",
            "GitHub Desktop",
            "--args",
            &format!("--user-data-dir={}", CLONE_DIR),
        ])
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
                .icon_as_template(false)
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
