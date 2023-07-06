// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod runner;
mod types;

use serde_json::{Value};
use log::{trace};
use env_logger;

#[tauri::command]
fn compile(path: &str) -> String {

    // Get the Clickscipt file.
    let input_path = String::from(path);

    let script = {
        // Load the first file into a string.
        let text = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    trace!("{}", script);

    return script.to_string();
}

fn main() {
    // Init logger
    env_logger::Builder::from_default_env().init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
