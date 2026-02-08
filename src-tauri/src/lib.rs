use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};

const CLONE_PREFIX: &str = "github-desktop-";
const SINGLETON_FILES: &[&str] = &["SingletonLock", "SingletonCookie", "SingletonSocket"];

fn cleanup_old_clones() {
    let Ok(entries) = fs::read_dir("/tmp") else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if !name.starts_with(CLONE_PREFIX) {
            continue;
        }
        let path = entry.path();
        let has_lock = path.join("SingletonLock").exists();
        if !has_lock {
            let _ = fs::remove_dir_all(&path);
        }
    }
}

fn source_user_data_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join("Library/Application Support/GitHub Desktop")
}

fn is_github_desktop_running() -> bool {
    Command::new("pgrep")
        .args(["-x", "GitHub Desktop"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn spawn_github_desktop() -> Result<(), String> {
    if !is_github_desktop_running() {
        Command::new("open")
            .args(["-a", "GitHub Desktop"])
            .spawn()
            .map_err(|e| format!("Failed to open GitHub Desktop: {}", e))?;
        return Ok(());
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    cleanup_old_clones();
    let clone_dir = format!("/tmp/{}{}", CLONE_PREFIX, ts);
    let source = source_user_data_dir();
    let status = Command::new("cp")
        .args(["-R", &source.to_string_lossy(), &clone_dir])
        .status()
        .map_err(|e| format!("Failed to copy user data: {}", e))?;
    if !status.success() {
        return Err("cp command failed".into());
    }
    let clone_path = PathBuf::from(&clone_dir);
    for file in SINGLETON_FILES {
        let _ = fs::remove_file(clone_path.join(file));
    }
    Command::new("open")
        .args([
            "-n",
            "-a",
            "GitHub Desktop",
            "--args",
            &format!("--user-data-dir={}", clone_dir),
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
