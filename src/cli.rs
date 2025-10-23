/// Módulo CLI - Interfaz de línea de comandos

/// Muestra el menú de ayuda con todos los comandos disponibles
pub fn print_help() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           TODO LIST - GESTOR DE TAREAS EN RUST             ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");
    println!("COMANDOS DISPONIBLES:\n");
    println!("  add <título>               - Crear una nueva tarea");
    println!("  list                       - Listar todas las tareas");
    println!("  get <id>                   - Obtener detalles de una tarea");
    println!("  update <id> <título>       - Actualizar título de tarea");
    println!("  status <id> <estado>       - Cambiar estado de tarea");
    println!("  delete <id>                - Eliminar una tarea");
    println!("  help                       - Mostrar esta ayuda\n");

    println!("ESTADOS VÁLIDOS: pendiente, en proceso, terminado, cancelado\n");

    println!("EJEMPLOS:\n");
    println!("  cargo run -- add \"Comprar leche\"");
    println!("  cargo run -- list");
    println!("  cargo run -- get 1");
    println!("  cargo run -- update 1 \"Nuevo título\"");
    println!("  cargo run -- status 1 \"en proceso\"");
    println!("  cargo run -- status 1 \"terminado\"");
    println!("  cargo run -- delete 1\n");
}
