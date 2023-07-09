use crate::types::types::{Arg, Convert};

/// This function transpiles a vector of Args into a string that is Javascript code
/// for printing items to the console.
///
/// # Arguments
///
/// * `args` - Vector of Args.
///
/// # Example
///
/// ```
/// let args = vec![
/// Arg {
///    arg_type: String::from("string"),
///    value: String::from("Hello,"),
/// },
/// Arg {
///    arg_type: String::from("string"),
///    value: String::from(" world!"),
/// }];
/// let result = print(args).unwrap();
/// assert_eq!(result, "console.log(\"Hello,\", \" world!\");\n");
/// ```
///
fn print(args: Vec<Arg>) -> Result<String, Box<dyn std::error::Error>> {
    let values: Result<Vec<_>, _> = args.into_iter().map(|arg| arg.convert()).collect();
    let values = values?;

    let js_code = format!("console.log({});\n", values.join(", "));

    Ok(js_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::types::Arg;

    #[test]
    fn test_print() {
        // Create a vector of Arg
        let args = vec![
            Arg {
                arg_type: String::from("string"),
                value: String::from("Hello,"),
                name: None,
            },
            Arg {
                arg_type: String::from("string"),
                value: String::from(" world!"),
                name: None,
            },
            Arg {
                arg_type: String::from("number"),
                value: String::from("57"),
                name: None,
            },
            Arg {
                arg_type: String::from("boolean"),
                value: String::from("true"),
                name: None,
            }
        ];

        // Test the print function
        let result = print(args).unwrap();
        assert_eq!(result, "console.log(\"Hello,\", \" world!\", 57, true);\n");
    }
}