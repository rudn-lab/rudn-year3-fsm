use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserInfoResult {
    Ok(UserInfo),
    NoSuchToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub rudn_id: String,
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub rudn_id: String,
}
