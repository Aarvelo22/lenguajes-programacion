

use std::fmt;
use std::str::FromStr;
use crate::errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {

    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pendiente",
            TaskStatus::InProgress => "en proceso",
            TaskStatus::Completed => "terminado",
            TaskStatus::Cancelled => "cancelado",
        }
    }

    pub fn all() -> Vec<TaskStatus> {
        vec![
            TaskStatus::Pending,
            TaskStatus::InProgress,
            TaskStatus::Completed,
            TaskStatus::Cancelled,
        ]
    }


    pub fn all_strings() -> Vec<&'static str> {
        Self::all().iter().map(|s| s.as_str()).collect()
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for TaskStatus {
    type Err = AppError;


    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pendiente" => Ok(TaskStatus::Pending),
            "en proceso" => Ok(TaskStatus::InProgress),
            "terminado" => Ok(TaskStatus::Completed),
            "cancelado" => Ok(TaskStatus::Cancelled),
            _ => Err(AppError::InvalidStatus(format!(
                "Estado inválido: '{}'. Estados válidos: {}",
                s,
                Self::all_strings().join(", ")
            ))),
        }
    }
}