use std::error::Error;
use rusty_v8 as v8;
use crate::types::types::Script;

pub fn compile(script: &Script) -> Result<String, Box<dyn Error>> {
    let mut code = String::new();

    // Iterate over the action_scripts and add them as imports
    for _action_script in &script.action_scripts {
        code.push_str(&format!("import * as actions from '{}';\n", String::from("../action/index.js").as_str()));
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

pub fn run(script: &Script) -> Result<(), Box<dyn Error>> {
    // Compile clickscript script into Javascript
    // NOTE: add support for TypeScript
    let code = compile(script).unwrap();
    // Initialize V8
    let platform = v8::new_default_platform(0, false);
    v8::V8::initialize_platform(platform.into());
    v8::V8::initialize();

    // Outer scope for isolate
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    let global_handle_scope = &mut v8::HandleScope::new(isolate);

    // Create a new context and enter it
    let context = v8::Context::new(global_handle_scope);

    {
        let scope = &mut v8::ContextScope::new(global_handle_scope, context);

        let source_code = v8::String::new(scope, &code).unwrap();
        
        let resource_name = v8::String::new(scope, "<module>").unwrap().into();

        let source_map_url = v8::String::new(scope, "").unwrap().into();

        // Set is_module to true in ScriptOrigin
        let script_origin = v8::ScriptOrigin::new(
            scope.as_mut(),
            resource_name,
            0,  // resource_line_offset
            0,  // resource_column_offset
            false,  // resource_is_shared_cross_origin
            -1,  // script_id (let V8 generate an ID)
            source_map_url,  // source_map_url
            false,  // resource_is_opaque
            false,  // is_wasm
            true  // is_module
        );
        let source = v8::script_compiler::Source::new(source_code, Some(&script_origin));
        let module = v8::script_compiler::compile_module(scope, source).unwrap();

        module
        .instantiate_module(scope, |_, _, _, m| Some(m))
        .ok_or("failed to instantiate module")?;
        module.evaluate(scope).ok_or("failed to evaluate module")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::{Action, Script};
    use serde_json::json;

    #[test]
    fn test_compile() {
        let script = Script {
            action_scripts: vec!["./src-tauri/src/actions/dist/index.js".to_string()],
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

        let expected_code = String::from("import * as actions from '../action/index.js';\n\n\nactions.log({\"level\":\"info\",\"message\":[\"Hello from Clickscript!\"]});");

        assert_eq!(compiled_code.trim(), expected_code.trim());
    }

    #[test]
    fn test_run() {
        let script = Script {
            action_scripts: vec!["./src-tauri/src/actions/dist/index.js".to_string()],
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

        let result = run(&script);

        if result.is_err() {
            eprintln!("Error running script: {:?}", result);
        }
    }
}