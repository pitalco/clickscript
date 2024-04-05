use core::fmt;

use serde::{Deserialize, Serialize};

pub trait Convert {
    fn convert(&self) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub index: i32,
    pub action: String,
    pub args: serde_json::Value,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub action_scripts: Vec<String>,
    pub script: Vec<Action>,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_string = self.args.to_string();

        write!(f, "Action {{ index: {}, action: '{}', args: [{}] }}", 
            self.index, self.action, args_string)
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let script_string = self.script.iter()
            .map(|action| action.to_string())  // Make sure `Action` also implements `Display`
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "Script: [{}]", script_string)
    }
}
