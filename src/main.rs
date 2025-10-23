/// Aplicación principal TODO List
/// Punto de entrada que maneja los argumentos de línea de comandos

use std::env;
use std::process;

mod task;
mod database;
mod status;
mod errors;
mod create;
mod read;
mod update;
mod delete;
mod cli;

use database::Database;
use cli::print_help;
use errors::AppError;

fn main() {
    // Obtener argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();

    // Inicializar conexión a la base de datos
    let mut db = match Database::new("tasks.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("✗ Error al conectar a la base de datos: {}", e);
            process::exit(1);
        }
    };

    // Validar que se proporcionó un comando
    if args.len() < 2 {
        print_help();
        return;
    }

    // Obtener el comando ingresado
    let command = &args[1];

    // Ejecutar el comando correspondiente
    if let Err(e) = execute_command(&mut db, command, &args) {
        eprintln!("✗ {}", e);
        process::exit(1);
    }
}

/// Ejecuta el comando especificado
fn execute_command(db: &mut Database, command: &str, args: &[String]) -> errors::AppResult<()> {
    match command {
        "add" => create::add_command(db, args),
        "list" => read::list_command(db),
        "get" => read::get_command(db, args),
        "update" => update::update_command(db, args),
        "status" => update::status_command(db, args),
        "delete" => delete::delete_command(db, args),
        "help" => {
            print_help();
            Ok(())
        }
        _ => Err(AppError::ValidationError(format!(
            "Comando desconocido: '{}'. Usa 'help' para ver los comandos disponibles",
            command
        ))),
    }
}
