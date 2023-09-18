use serde::{Deserialize, Serialize};
use std::io::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    id: String,
    content: String,
}

impl Data {
    pub fn new(content: String) -> Result<Self> {
        return Ok(Data {
            id: uuid::Uuid::new_v4().to_string(),
            content,
        });
    }
}
