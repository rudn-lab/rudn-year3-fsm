use fsm::fsm::{FSMError, FSMOutput, StateMachine};
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
    pub legend: String,
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
    pub script: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct UserTaskSubmissions {
    pub latest_submission: Option<UserTaskSubmission>,
    pub latest_ok_submission: Option<UserTaskSubmission>,
    pub submissions: Vec<UserTaskSubmission>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserTaskSubmission {
    pub id: i64,
    pub task_id: i64,
    pub when_unix_time: i64,
    pub solution: StateMachine,
    pub verdict: SubmissionVerdict,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SubmissionVerdict {
    /// All N tests have passed
    Ok(usize),
    /// Some tests have not passed: the seed for the first failed one is provided
    WrongAnswer {
        total_tests: usize,
        successes: usize,
        first_failure_seed: i64,
        first_failure_expected_result: FSMOutput,
    },

    /// The state machine is invalid
    InvalidFSM(FSMError),

    /// The task is invalid -- this is the jury's fault
    TaskInternalError(String),
}
