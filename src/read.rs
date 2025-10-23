/// Módulo Read - Leer y listar tareas

use crate::database::Database;
use crate::errors::AppResult;

/// Maneja el comando 'list' para mostrar todas las tareas
pub fn list_command(db: &Database) -> AppResult<()> {
    let tasks = db.read_all_tasks()?;

    if tasks.is_empty() {
        println!("No hay tareas");
        return Ok(());
    }

    println!("\n{:<5} {:<25} {:<15} {:<20} {:<20}", "ID", "Título", "Estado", "Creada", "Completada");
    println!("{}", "-".repeat(90));

    for task in tasks {
        let completed = task.completed_at.as_deref().unwrap_or("-");
        println!(
            "{:<5} {:<25} {:<15} {:<20} {:<20}",
            task.id,
            truncate(&task.title, 23),
            task.status,
            task.created_at,
            completed
        );
    }
    println!();
    Ok(())
}

/// Maneja el comando 'get' para obtener detalles de una tarea específica
pub fn get_command(db: &Database, args: &[String]) -> AppResult<()> {
    if args.len() < 3 {
        return Err(crate::errors::AppError::MissingArgument("ID de la tarea".to_string()));
    }

    let id: u32 = args[2].parse()
        .map_err(|_| crate::errors::AppError::ParseError(
            format!("El ID '{}' no es un número válido", args[2])
        ))?;

    let task = db.read_task(id)?;

    println!("\n{}", "=".repeat(50));
    println!("ID: {}", task.id);
    println!("Título: {}", task.title);
    println!("Estado: {}", task.status);
    println!("Creada: {}", task.created_at);
    if let Some(completed) = task.completed_at {
        println!("Completada: {}", completed);
    }
    println!("{}\n", "=".repeat(50));
    Ok(())
}

/// Función auxiliar para truncar texto largo
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len - 3])
    } else {
        s.to_string()
    }
}

