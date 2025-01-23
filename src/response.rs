use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct NewInstanceResponse {
    pub id: String,
    pub name: String,
    pub zone: String,
    pub operation_type: String,
    pub target_link: String,
    pub user: String,
    pub status: String,
    pub insert_time: String,
    pub self_link: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LaunchInstanceResponse {
    pub id: String,
    pub name: String,
    pub zone: String,
    pub operation_type: String,
    pub target_link: String,
    pub status: String,
    pub process: usize,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ShutdownInstanceResponse {
    pub id: String,
    pub name: String,
    pub zone: String,
    pub operation_type: String,
    pub target_link: String,
    pub status: String,
    pub process: usize,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DeleteInstanceResponse {
    pub id: String,
    pub name: String,
    pub zone: String,
    pub operation_type: String,
    pub target_link: String,
    pub status: String,
    pub process: usize,
    pub start_time: String,
    pub end_time: String,
}

