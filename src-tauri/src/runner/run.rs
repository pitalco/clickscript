use deno_core::{error::AnyError, JsRuntime};
use serde_json::Value;
use crate::types::types::Script;

pub fn compile(script: &Script) -> Result<String, AnyError> {
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
        let action_code = generate_action_code(action_name, args);

        code.push_str(&action_code);
        code.push_str("\n\n");
    }

    Ok(code)
}

fn generate_action_code(action_name: &str, args: &Value) -> String {
    let args_str = serde_json::to_string(args).unwrap();
    format!("actions.{}({});", action_name, args_str)
}

pub fn run(script: &Script) {
    let code = compile(script).unwrap();
    let mut runtime = JsRuntime::new(Default::default());
    let result = runtime.execute_script("<anon>", code);
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::{Action, Script};
    use serde_json::json;

    #[test]
    fn test_compile() {
        let script = Script {
            action_scripts: vec!["./src-tauri/src/actions/index.ts".to_string()],
            script: vec![
                Action {
                    index: 1,
                    action: "log".to_string(),
                    args: json!({
                        "message": "Hello from Clickscript!",
                        "level": "info"
                    }),
                },
            ],
        };

        let compiled_code = compile(&script).unwrap();

        let expected_code = String::from("import * as actions from './src-tauri/src/actions/index.ts';\n\nactions.log({\"message\":\"Hello from Clickscript!\",\"level\":\"info\"});");

        assert_eq!(compiled_code.trim(), expected_code.trim());
    }

    #[test]
    fn test_run() {
        let script = Script {
            action_scripts: vec!["./src-tauri/src/actions/index.ts".to_string()],
            script: vec![
                Action {
                    index: 1,
                    action: "log".to_string(),
                    args: json!({
                        "message": "Hello from Clickscript!",
                        "level": "info"
                    }),
                },
            ],
        };

        run(&script);
    }
}