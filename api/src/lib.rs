use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserInfo {
    Ok {
        token: String,
        name: String,
        rudn_id: String,
    },
    NoSuchToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub rudn_id: String,
}
