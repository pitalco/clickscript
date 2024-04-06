use core::fmt;
use std::{error::Error, fs, path::PathBuf};
use log::{error, info};
use rusty_v8 as v8;
use serde_json::to_string;
use crate::types::types::Script;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

pub struct ClicksciptLog {
    pub message: String,
    pub level: String,
}

impl fmt::Display for ClicksciptLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.level, self.message)
    }
}

impl ClicksciptLog {
    pub fn new(message: String, level: String) -> ClicksciptLog {
        ClicksciptLog {
            message,
            level,
        }
    }
}

// We set a static global variable to store the logs from v8. We use a Mutex to ensure that the logs are thread-safe and an Arc to share the logs across threads.
// We also use the once_cell crate to lazily initialize the global variable to save CPU time if we never need it.
static GLOBAL_LOGS: Lazy<Arc<Mutex<Vec<ClicksciptLog>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

fn log_callback(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _: v8::ReturnValue) {
    let message = args.get(0).to_string(scope).unwrap().to_rust_string_lossy(scope);
    
    let logs = GLOBAL_LOGS.clone();
    let mut logs = logs.lock().unwrap();
    logs.push(ClicksciptLog::new(message, "info".to_string()));
}

fn error_callback(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _: v8::ReturnValue) {
    for i in 0..args.length() {
        let arg = args.get(i);
        let info = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
        let logs = GLOBAL_LOGS.clone();
        let mut logs = logs.lock().unwrap();
        logs.push(ClicksciptLog::new(info, "error".to_string()));
    }
    return;
}

pub fn compile(script: &Script) -> Result<String, Box<dyn Error>> {
    let mut code = String::new();

    // Iterate over the action_scripts and add them as imports
    for action_script in &script.action_scripts {
        let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(action_script);
        let path = PathBuf::from(script_path);
        // Check if the action script exists
        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.is_file() {
                // If it does, add it as an import
                code.push_str(&format!("import * as actions from {:#?};", action_script));
            } else {
                // Handle the case when the action script is not a file
                return Err(format!("Action script '{:#?}' is not a file", action_script).into());
            }
        } else {
            // Handle the case when the action script does not exist
            return Err(format!("Action script '{:#?}' does not exist", action_script).into());
        }
    }

    // Iterate over the script actions and generate the corresponding code
    for action in &script.script {
        let action_name = &action.action;
        let args = &action.args;

        // Serialize `args` to a JSON string
        let args_json = to_string(args).unwrap_or_else(|_| "{}".to_string());

        // Generate the code for the action
        // Directly inject the JSON string as part of the JavaScript code
        let action_code = format!("actions.{}({});", action_name, args_json);

        code.push_str("\n\n");
        code.push_str(&action_code);
    }

    Ok(code)
}

pub fn run(script: &Script) {
    env_logger::init();

    // Initialize V8
    let platform = v8::new_default_platform(0, false);
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // Create a new Isolate and a HandleScope
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let handle_scope = &mut v8::HandleScope::new(&mut isolate);
    
    // Create a new context
    let context = v8::Context::new(handle_scope);
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let global = context.global(scope);

    // Create a new function in the V8 context
    let log_function = v8::Function::new(scope, log_callback).unwrap();
    let log_key = v8::String::new(scope, "log").unwrap();

    // Create a new function in the V8 context
    let error_function = v8::Function::new(scope, error_callback).unwrap();
    let error_key = v8::String::new(scope, "error").unwrap();

    // Set the console object in global. This is a way for the JavaScript code to call into Rust logging, including panic errors.
    let console = v8::Object::new(scope);
    // Set console.log
    console.set(scope, log_key.into(), log_function.into()).unwrap();
    // Set console.error
    console.set(scope, error_key.into(), error_function.into()).unwrap();
    let console_key = v8::String::new(scope, "console").unwrap();
    global.set(scope, console_key.into(), console.into()).unwrap();

    // Compile module
    let raw_code = compile(script);
    let code = format!("try {{console.log({:?});\n{:?}\n}} catch (error) {{\n\tconsole.error(error);\n}}", "Hello from JS", raw_code.unwrap());
    let source_code = v8::String::new(scope, code.as_str()).unwrap();
    let resource_name = v8::String::new(scope, "clickscript_module").unwrap().into();
    let source_map_url = v8::String::new(scope, "").unwrap().into();

    let script_origin = v8::ScriptOrigin::new(
        scope,
        resource_name,
        0, 0, false, -1, source_map_url, false, false, true
    );
    let source = v8::script_compiler::Source::new(source_code, Some(&script_origin));
    let module = v8::script_compiler::compile_module(scope, source).unwrap();

    // Instantiate the module and evaluate it
    module.instantiate_module(scope, |_, _, _, m| Some(m)).unwrap();
    module.evaluate(scope).unwrap();
    let logs = GLOBAL_LOGS.lock().unwrap();
    for log in logs.iter() {
        match log.level.as_str() {
            "info" => info!("{}", log.message),
            "error" => error!("{}", log.message),
            _ => info!("{}", log.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use super::*;
    use crate::types::types::{Action, Script};
    use serde_json::json;

    #[test]
    fn test_compile() {
        let script = Script {
            action_scripts: vec!["./packages/actions/dist/index.js".to_string()],
            script: vec![
                Action {
                    index: 1,
                    action: "log".to_string(),
                    args: json!({
                        "message": [
                            "Hello from Clickscript!"
                        ],
                        "level": "info"
                    }),
                },
            ],
        };

        let compiled_code = compile(&script).unwrap(); 

        let file = File::create("test.js");
        let result = file.unwrap().write_all(compiled_code.as_bytes());
        result.unwrap();

        let expected_code = String::from("import * as actions from \"./packages/actions/dist/index.js\";\n\nactions.log({\"level\":\"info\",\"message\":[\"Hello from Clickscript!\"]});");

        assert_eq!(compiled_code.trim(), expected_code.trim());
    }

    #[test]
    fn test_run() {
        let script = Script {
            action_scripts: vec!["./packages/actions/dist/index.js".to_string()],
            script: vec![
                Action {
                    index: 1,
                    action: "log".to_string(),
                    args: json!({
                        "message": [
                            "Hello from Clickscript!"
                        ],
                        "level": "info"
                    }),
                },
            ],
        };

        run(&script);
    }
}