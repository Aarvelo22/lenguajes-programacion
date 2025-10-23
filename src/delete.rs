// src/delete.rs
/// Módulo Delete - Eliminar tareas

use crate::database::Database;
use crate::errors::{AppError, AppResult};

/// Maneja el comando 'delete' para eliminar una tarea
/// 
/// # Ejemplo de uso en terminal:
/// cargo run -- delete 1
pub fn delete_command(db: &mut Database, args: &[String]) -> AppResult<()> {
    if args.len() < 3 {
        return Err(AppError::MissingArgument("ID de la tarea".to_string()));
    }

    let id: u32 = args[2].parse()
        .map_err(|_| AppError::ParseError(
            format!("El ID '{}' no es un número válido", args[2])
        ))?;

    db.delete_task(id)?;

    println!("✓ Tarea {} eliminada", id);
    Ok(())
}