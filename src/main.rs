

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

    let args: Vec<String> = env::args().collect();

    let mut db = match Database::new("tasks.db") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("✗ Error al conectar a la base de datos: {}", e);
            process::exit(1);
        }
    };

    if args.len() < 2 {
        print_help();
        return;
    }


    let command = &args[1];


    if let Err(e) = execute_command(&mut db, command, &args) {
        eprintln!("✗ {}", e);
        process::exit(1);
    }
}

fn execute_command(db: &mut Database, command: &str, args: &[String]) -> errors::AppResult<()> {
    match command {
        "add" => create::add_command(db, args),
        "list" => read::list_command(db),
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

