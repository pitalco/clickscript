use crate::actions::handler::handler;
use crate::types::types::{Action, Arg};

pub fn for_loop(args: Vec<Arg>, children: Option<Vec<Action>>) -> Result<String, Box<dyn std::error::Error>> {
    let loop_var = args.iter().filter(|x| x.name == Some("var_name".to_string())).next();

    let mut loop_body = String::new();
    for action in children.unwrap() {
        let func = match handler(&action) {
            Ok(func) => func,
            Err(e) => panic!("{}", e),
        };
        let code = func(action.args, action.children);
        let code_string = match code {
            Ok(code) => code,
            Err(e) => panic!("{}", e),
        };
        loop_body.push_str(&code_string);
        loop_body.push_str("\n");
    }

    let js_code = format!("for (let item of {}) {{\n{}\n}}\n", 
                           loop_var.unwrap().value, 
                           loop_body);

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
                value: String::from("items"),
                name: Some(String::from("var_name")),
            }
        ];
        
        // Create a vector of Action
        let actions = vec![
            Action {
                index: 1,
                action: String::from("print"),
                args: args.clone(),
                children: Some(vec![]),
            }
        ];
        
        // Test the create_for_loop function
        let result = for_loop(args, Some(actions)).unwrap();
        assert_eq!(result, "for (let i of array) {\nconsole.log(i);\n}\n");
    }
}