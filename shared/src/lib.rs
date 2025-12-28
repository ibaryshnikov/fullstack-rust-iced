use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Data {
    pub a: i32,
    pub b: i32,
}
