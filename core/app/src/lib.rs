// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod ipc;

// TODO: Can't add it to macro with fully qualified path, better way of registering IPC commands?
use crate::ipc::{get_envs, greet};

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_envs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
