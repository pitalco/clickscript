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
            "string" => Ok(self.value.to_owned()),
            "number" => Ok(self.value.to_owned()),
            _ => Err(Box::new(std::fmt::Error)),
        }
    }
}