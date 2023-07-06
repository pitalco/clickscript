use crate::types::types::{Arg, Convert};

fn print(args: Vec<Arg>) -> Result<String, Box<dyn std::error::Error>> {
    let mut js_code = String::new();
    for arg in args {
        js_code += &format!("console.log('{:?}');\n", arg.convert()?);
    }

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
                value: String::from("Hello, world!"),
            },
        ];

        // Test the print function
        let result = print(args).unwrap();
        assert_eq!(result, "console.log('Hello, world!');\n");
    }
}