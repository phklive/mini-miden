use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub proof: Vec<u8>,
    pub hash: Vec<u64>,
    pub outputs_stack: Vec<u64>,
    pub outputs_overflow: Vec<u64>,
}
