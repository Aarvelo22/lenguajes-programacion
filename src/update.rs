/// Módulo Update - Actualizar tareas

use crate::database::Database;
use crate::status::TaskStatus;
use crate::errors::{AppError, AppResult};

/// Maneja el comando 'update' para actualizar el título de una tarea
pub fn update_command(db: &mut Database, args: &[String]) -> AppResult<()> {
    if args.len() < 4 {
        return Err(AppError::MissingArgument(
            "ID y nuevo título".to_string(),
        ));
    }

    let id: u32 = args[2].parse()
        .map_err(|_| AppError::ParseError(
            format!("El ID '{}' no es un número válido", args[2])
        ))?;

    let new_title = &args[3];
    db.update_task(id, new_title)?;

    println!("✓ Tarea {} actualizada", id);
    Ok(())
}

/// Maneja el comando 'status' para cambiar el estado de una tarea
pub fn status_command(db: &mut Database, args: &[String]) -> AppResult<()> {
    if args.len() < 4 {
        return Err(AppError::MissingArgument(
            "ID y nuevo estado".to_string(),
        ));
    }

    let id: u32 = args[2].parse()
        .map_err(|_| AppError::ParseError(
            format!("El ID '{}' no es un número válido", args[2])
        ))?;

    let new_status = &args[3];
    let status = new_status.parse::<TaskStatus>()?;

    db.update_task_status(id, status)?;

    if status == TaskStatus::Completed {
        println!("✓ Tarea {} marcada como terminada", id);
    } else {
        println!("✓ Estado de tarea {} actualizado a: {}", id, status);
    }
    Ok(())
}
