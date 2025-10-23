// src/errors.rs
/// Módulo Errors - Manejo de errores personalizado

use std::fmt;

/// Enum que representa los posibles errores de la aplicación
#[derive(Debug)]
pub enum AppError {
    /// Error de base de datos SQLite
    DatabaseError(String),
    /// Error de validación de entrada
    ValidationError(String),
    /// Error de parseo de argumentos
    ParseError(String),
    /// Tarea no encontrada
    TaskNotFound(u32),
    /// Argumento faltante
    MissingArgument(String),
    /// Estado inválido
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

/// Convierte rusqlite::Error a AppError
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

/// Tipo Result personalizado para la aplicación
pub type AppResult<T> = Result<T, AppError>;