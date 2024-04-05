use std::{error::Error, fs, path::PathBuf};
use rusty_v8 as v8;
use crate::types::types::Script;

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

        // Generate the code for the action
        let args_str = serde_json::to_string(args).unwrap();
        let action_code = format!("actions.{}({});", action_name, args_str);

        code.push_str("\n\n");
        code.push_str(&action_code);
    }

    Ok(code)
}

pub fn run(script: &Script) {
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

    // Define the log callback function
    let log_callback = |scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _: v8::ReturnValue| {
        for i in 0..args.length() {
            let arg = args.get(i);
            let str = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            println!("{}", str);
        }
    };
    // Create a new function in the V8 context
    let log_function = v8::Function::new(scope, log_callback).unwrap();
    let log_key = v8::String::new(scope, "log").unwrap();

    // Define the error callback function
    let error_callback = |scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _: v8::ReturnValue| {
        for i in 0..args.length() {
            let arg = args.get(i);
            let str = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            println!("{}", str);
        }
    };
    // Create a new function in the V8 context
    let error_function = v8::Function::new(scope, error_callback).unwrap();

    // Set the console object in global. This is a way for the JavaScript code to call into Rust logging, including panic errors.
    let console = v8::Object::new(scope);
    console.set(scope, log_key.into(), log_function.into());
    let console_key = v8::String::new(scope, "console").unwrap();
    global.set(scope, console_key.into(), console.into()).unwrap();

    // Set error function in global
    let error_key = v8::String::new(scope, "error").unwrap();
    global.set(scope, error_key.into(), error_function.into()).unwrap();

    // Compile module
    let code = compile(script);
    let source_code = v8::String::new(scope, code.unwrap().as_str()).unwrap();
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