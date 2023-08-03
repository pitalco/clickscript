use crate::types::types::{Arg, Action};
use crate::actions::handler::handler;

pub fn create_function(args: Vec<Arg>, children: Option<Vec<Action>>) -> Result<String, Box<dyn std::error::Error>> {
    let func_name = args.iter().filter(|x| x.name == Some("function_name".to_string())).next();

    let mut function_body = String::new();
    for action in children.unwrap() {
        let func = handler(&action).ok().unwrap();
        let code = func(action.args, action.children);
        function_body.push_str(&code.unwrap());
        function_body.push_str("\n");
    }

    let js_code = format!("const {} = () => {{\n{}}}\n", func_name.clone().unwrap().value, function_body);

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
            name: Some(String::from("function_name")),
        };
        
        // Create a vector of Action
        let actions = vec![
            Action {
                index: 1,
                action: String::from("create_variable"),
                args: args.clone(),
                children: Some(vec![]),
            }
        ];
        
        // Test the create_function function
        let result = create_function(vec!(func_name), Some(actions)).unwrap();
        assert_eq!(result, "function myFunction() {\nlet name = \"Hello!\";\n\n}\n");
    }
}
