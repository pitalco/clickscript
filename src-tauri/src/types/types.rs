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