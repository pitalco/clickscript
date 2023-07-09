use crate::types::types::{Arg, Action, Convert};

fn create_for_loop(loop_var: Arg, actions: Vec<Action>) -> Result<String, Box<dyn std::error::Error>> {
    let loop_variable = match loop_var.name {
        Some(name) => name,
        None => panic!("Loop variable name is required."),
    };

    let mut loop_body = String::new();
    for action in actions {
        for arg in action.args {
            loop_body.push_str(&arg.convert().unwrap());
            loop_body.push_str(";\n");
        }

        if !action.children.is_empty() {
            loop_body.push_str(&create_for_loop(loop_var.clone(), action.children)?);
        }
    }

    let js_code = format!("for(let {} of predefinedArray) {{\n{}}}\n", loop_variable, loop_body);

    Ok(js_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::{Arg, Action};

    #[test]
    fn test_create_for_loop() {
        // Create a vector of Arg
        let args = vec![
            Arg {
                arg_type: String::from("var"),
                value: String::from("name"),
                name: Some(String::from("var_name")),
            },
            Arg {
                arg_type: String::from("string"),
                value: String::from("Hello!"),
                name: Some(String::from("value")),
            }
        ];
        
        let loop_var = Arg {
            arg_type: String::from("string"),
            value: String::from("item"),
            name: Some(String::from("loop_var")),
        };
        
        // Create a vector of Action
        let actions = vec![
            Action {
                index: 1,
                action: String::from("create_variable"),
                args: args.clone(),
                children: vec![],
            }
        ];
        
        // Test the create_for_loop function
        let result = create_for_loop(loop_var, actions).unwrap();
        assert_eq!(result, "for(let item of predefinedArray) {\nlet name = \"Hello!\";\n}\n");
    }
}