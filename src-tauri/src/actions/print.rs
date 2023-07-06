use crate::types::types::{Arg, Convert};

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
            },
            Arg {
                arg_type: String::from("string"),
                value: String::from(" world!"),
            },
            Arg {
                arg_type: String::from("number"),
                value: String::from("57"),
            },
            Arg {
                arg_type: String::from("boolean"),
                value: String::from("true"),
            }
        ];

        // Test the print function
        let result = print(args).unwrap();
        assert_eq!(result, "console.log(\"Hello,\", \" world!\", 57, true);\n");
    }
}