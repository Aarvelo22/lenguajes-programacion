// src/errors.rs


use std::fmt;

#[derive(Debug)]
pub enum AppError {

    DatabaseError(String),
    ValidationError(String),
    ParseError(String),
    TaskNotFound(u32),
    MissingArgument(String),
    InvalidStatus(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Error de base de datos: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Error de validación: {}", msg),
            AppError::ParseError(msg) => write!(f, "Error de parseo: {}", msg),
            AppError::TaskNotFound(id) => write!(f, "Tarea con ID {} no encontrada", id),
            AppError::MissingArgument(arg) => write!(f, "Argumento faltante: {}", arg),
            AppError::InvalidStatus(status) => write!(f, "Estado inválido: {}", status),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;