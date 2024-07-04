// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::process::{Command, Stdio};
// use std::fs;

use tauri::Window;

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move{
                let exe_path = app_handle
                    .path_resolver()
                    .resolve_resource("backend/uvicorn_app.exe")
                    .expect("failed to resolve resource")
                    .to_string_lossy()
                    .to_string();
                Command::new(exe_path)
                    .spawn()
                    .expect("Failed to start backend");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![close_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



#[tauri::command]
fn close_window(window: Window) {
    // Kill the backend process
    if cfg!(target_os = "windows") {
        Command::new("taskkill")
            .args(&["/IM", "uvicorn_app.exe", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .expect("Failed to kill backend process");
    } else {
        Command::new("pkill")
            .arg("uvicorn_app")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .expect("Failed to kill backend process");
    }

    // Remove the specific file
    /*let file_path = window
        .app_handle()
        .path_resolver()
        .resolve_resource("uvicorn_app.lock")
        .expect("failed to resolve resource");

    if let Err(e) = fs::remove_file(file_path) {
        eprintln!("Failed to remove file: {}", e);
    }*/

    // Close the application window
    window.close().expect("failed to close window");
}
