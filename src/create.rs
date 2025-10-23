// src/create.rs
/// Módulo Create - Crear nuevas tareas

use crate::database::Database;
use crate::errors::{AppError, AppResult};

/// Maneja el comando 'add' para crear una nueva tarea
///
/// # Ejemplo de uso en terminal:
/// cargo run -- add "Comprar leche"
pub fn add_command(db: &mut Database, args: &[String]) -> AppResult<()> {
    if args.len() < 3 {
        return Err(AppError::MissingArgument(
            "descripción de la tarea".to_string(),
        ));
    }

    let description = &args[2];
    let id = db.create_task(description)?;

    println!("✓ Tarea creada con ID: {}", id);
    Ok(())
}