// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod runner;
mod types;

use log::trace;
use env_logger;
use crate::types::types::Script;

#[tauri::command]
fn compile(path: &str) -> String {

    // Get the Clickscipt file.
    let input_path = String::from(path);

    // Load the first file into a string.
    let file_content = match std::fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(err) => {
            return format!("Error reading file: {}: {}", input_path, err);
        }
    };

    // Parse the string into a Script object.
    let script_res = match serde_json::from_str(&file_content) {
        Ok(content) => content,
        Err(err) => {
            return format!("Error parsing code into a Clickscript object: {}: {}", file_content, err);
        }
    };

    let code = runner::run::compile(&script_res);

    trace!("{:?}", code);

    return code.unwrap();
}

#[tauri::command]
fn compile_run(path: &str) {

    // Get the Clickscipt file.
    let input_path = String::from(path);

    let script = {
        // Load the first file into a string.
        let script: String = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Script>(&script).unwrap()
    };

    runner::run::run(&script);
}

fn main() {
    // Init logger
    env_logger::Builder::from_default_env().init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile, compile_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
