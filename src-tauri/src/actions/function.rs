use crate::types::types::{Arg, Action, Convert};

fn create_function(func_name: Arg, actions: Vec<Action>) -> Result<String, Box<dyn std::error::Error>> {
    let function_name = match func_name.name {
        Some(name) => name,
        None => panic!("Function name is required."),
    };

    let mut function_body = String::new();
    for action in actions {
        for arg in action.args {
            function_body.push_str(&arg.convert().unwrap());
            function_body.push_str(";\n");
        }

        if !action.children.is_empty() {
            function_body.push_str(&create_function(func_name.clone(), action.children)?);
        }
    }

    let js_code = format!("function {}() {{\n{}}}\n", function_name, function_body);

    Ok(js_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::{Arg, Action};

    #[test]
    fn test_create_function() {
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
        
        let func_name = Arg {
            arg_type: String::from("string"),
            value: String::from("myFunction"),
            name: Some(String::from("func_name")),
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
        
        // Test the create_function function
        let result = create_function(func_name, actions).unwrap();
        assert_eq!(result, "function myFunction() {\nlet name = \"Hello!\";\n}\n");
    }
}
