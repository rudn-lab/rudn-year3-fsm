use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserInfoResult {
    Ok(UserInfo),
    NoSuchToken,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub name: String,
    pub rudn_id: String,
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RegisterRequest {
    pub name: String,
    pub rudn_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskGroupInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub tasks: Vec<SmallTaskInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SmallTaskInfo {
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TaskInfo {
    pub name: String,
    pub slug: String,
    pub legend: String,
    pub testgen_script: String,
    pub testchk_script: String,
}
