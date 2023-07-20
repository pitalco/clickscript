use core::fmt;

use serde::{Deserialize, Serialize};

pub trait Convert {
    fn convert(&self) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Arg {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub value: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub index: i32,
    pub action: String,
    pub args: Vec<Arg>,
    pub children: Option<Vec<Action>>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub script: Vec<Action>,
}


impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_string = self.name.as_ref()
            .map(|name| format!("'{}'", name))
            .unwrap_or_else(|| "None".to_string());

        write!(f, "Arg {{ type: '{}', value: '{}', name: {} }}",
            self.arg_type, self.value, name_string)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_string = self.args.iter()
            .map(|arg| arg.to_string())  // Make sure `Arg` also implements `Display`
            .collect::<Vec<_>>()
            .join(", ");

        let children_string = self.children.as_ref()
            .map(|children| children.iter()
                .map(|child| child.to_string())
                .collect::<Vec<_>>()
                .join(", "))
            .unwrap_or_else(|| "None".to_string());

        write!(f, "Action {{ index: {}, action: '{}', args: [{}], children: [{}] }}", 
            self.index, self.action, args_string, children_string)
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

impl Convert for Arg {
    fn convert(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.arg_type.as_str() {
            "string" => Ok(format!("\"{}\"", self.value)),
            "number" => {
                // Attempt to parse the value as a number to validate it
                self.value.parse::<f64>()
                    .map_err(|_| "Invalid number".into())
                    .map(|_| self.value.to_owned())
            },
            "var" => Ok(format!("{}", self.value)),
            "boolean" => {
                // Validate that the value is a boolean
                match self.value.to_lowercase().as_str() {
                    "true" | "false" => Ok(self.value.to_owned()),
                    _ => Err("Invalid boolean".into())
                }
            },
            "list" => {
                // Attempt to parse the value as a list to validate it
                let value: Result<serde_json::Value, _> = serde_json::from_str(&self.value);
                match value {
                    Ok(v) => Ok(serde_json::to_string_pretty(&v).unwrap_or_else(|_| self.value.to_owned())),
                    Err(_) => Err("Invalid list".into())
                }
            },
            "object" => {
                // Attempt to parse the value as JSON to validate it
                let value: Result<serde_json::Value, _> = serde_json::from_str(&self.value);
                match value {
                    Ok(v) => Ok(serde_json::to_string_pretty(&v).unwrap_or_else(|_| self.value.to_owned())),
                    Err(_) => Err("Invalid JSON".into())
                }
            },
            _ => Err("Unsupported type".into()),
        }
    }
}