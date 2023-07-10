use crate::types::types::{Action, Arg};
use crate::actions::print;
use crate::actions::function;
use crate::actions::loops;
use crate::actions::variables;

pub fn handler(action: &Action) -> Result<fn(args: Vec<Arg>, children: Option<Vec<Action>>) -> Result<String, Box<dyn std::error::Error>>, Box<dyn std::error::Error>> {
    match action.action.as_str() {
       "print" => Ok(print::print),
       "create_function" => Ok(function::create_function),
       "create_for" => Ok(loops::for_loop),
       "create_variable" => Ok(variables::variable),
       _ => Err(format!("Handler function {}() is not supported in the handler.", action.action.to_string()).into()),
    }
}