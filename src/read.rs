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
            task.title,
            task.status,
            task.created_at,
            completed
        );
    }
    println!();
    Ok(())
}
