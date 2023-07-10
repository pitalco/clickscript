use crate::types::types::{Arg, Convert, Action};

/// This function transpiles a vector of Args into a string that is Javascript code
/// for setting a variable to certain values.
///
/// # Arguments
///
/// * `variable_name` - A string that will be used as the variable name in the generated Javascript code.
/// * `args` - Vector of Args.
///
/// # Example
///
/// ```
/// let args = vec![
/// Arg {
///     arg_type: String::from("string"),
///     value: String::from("name,"),
///     name: Some(String::from("var_name")),
/// },
/// Arg {
///     arg_type: String::from("string"),
///     value: String::from("Hello!,"),
///     name: Some(String::from("value")),
/// }];
/// let result = variable("greeting", args).unwrap();
/// assert_eq!(result, "let greeting = [\"Hello,\", \" world!\"];\n");
/// ```
///
pub fn variable(args: Vec<Arg>, _children: Option<Vec<Action>>) -> Result<String, Box<dyn std::error::Error>> {
    let variable_name = args.iter().filter(|x| x.name == Some("var_name".to_string())).next();
    let var = match variable_name {
        Some(var_ref) => var_ref.clone(),
        None => panic!("Could not convert variable_name to Arg type.")
    };

    let variable_value = args.iter().filter(|x| x.name == Some("value".to_string())).next();
    let value: Arg = match variable_value {
        Some(value_ref) => value_ref.clone(),
        None => panic!("Could not convert value to Arg type.")
    };

    let js_code = format!("let {} = {};\n", var.convert().unwrap().to_string(), value.convert().unwrap().to_string());
    
    Ok(js_code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::Arg;

    #[test]
    fn test_set_variable() {
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

        // Test the variable function
        let result = variable(args, None).unwrap();
        println!("{:?}", result.as_str());
        assert_eq!(result, "let name = \"Hello!\";\n");
    }
}