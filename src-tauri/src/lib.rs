use std::path::{Path, PathBuf};

pub mod file_io;
pub mod helpers;
pub mod models;

use crate::file_io::read_dir;
use crate::helpers::{create_report, create_report_with_path};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start(target_path: Option<&str>) -> String {
    match target_path {
        Some(path) => {
            println!("path {}", path);
            create_report_with_path(&PathBuf::new().join(path))
        }
        _ => {
            let all_files = read_dir(Path::new("input")).unwrap();
            
            for path in &all_files {
                create_report(path);
            }
            
            all_files[0].to_str().unwrap().to_string()
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
