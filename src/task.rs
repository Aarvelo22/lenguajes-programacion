

use crate::status::TaskStatus;


#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: TaskStatus,
    pub created_at: String,
    pub completed_at: Option<String>,
}

impl Task {

    pub fn new(id: u32, title: String, status: TaskStatus, created_at: String, completed_at: Option<String>) -> Self {
        Task {
            id,
            title,
            status,
            created_at,
            completed_at,
        }
    }
}


