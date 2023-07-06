use serde::{Deserialize, Serialize};

pub trait Convert {
    fn convert(&self) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arg {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    pub index: i32,
    pub action: String,
    pub args: Vec<Arg>,
    pub children: Vec<Action>,
}

impl Convert for Arg {
    fn convert(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.arg_type.as_str() {
            "string" => Ok(format!("{}", self.value)),
            "number" => {
                // Attempt to parse the value as a number to validate it
                self.value.parse::<f64>()
                    .map_err(|_| "Invalid number".into())
                    .map(|_| self.value.to_owned())
            },
            "boolean" => {
                // Validate that the value is a boolean
                match self.value.to_lowercase().as_str() {
                    "true" | "false" => Ok(self.value.to_owned()),
                    _ => Err("Invalid boolean".into())
                }
            },
            "list" => {
                // Attempt to parse the value as a list to validate it
                serde_json::from_str::<serde_json::Value>(&self.value)
                    .map_err(|_| "Invalid list".into())
                    .map(|_| self.value.to_owned())
            },
            "object" => {
                // Attempt to parse the value as JSON to validate it
                serde_json::from_str::<serde_json::Value>(&self.value)
                    .map_err(|_| "Invalid JSON".into())
                    .map(|_| self.value.to_owned())
            },
            _ => Err("Unsupported type".into()),
        }
    }
}