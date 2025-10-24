

use rusqlite::{Connection, Result as SqliteResult, params};
use crate::task::Task;
use crate::status::TaskStatus;
use crate::errors::{AppError, AppResult};


pub struct Database {
    connection: Connection,
}

impl Database {

    pub fn new(db_path: &str) -> AppResult<Self> {
        let connection = Connection::open(db_path)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let db = Database { connection };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> AppResult<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pendiente',
                created_at TEXT NOT NULL,
                completed_at TEXT
            )",
            [],
        ).map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub fn create_task(&mut self, title: &str) -> AppResult<u32> {
        if title.trim().is_empty() {
            return Err(AppError::ValidationError(
                "El título de la tarea no puede estar vacío".to_string(),
            ));
        }

        let created_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        self.connection.execute(
            "INSERT INTO tasks (title, status, created_at, completed_at) VALUES (?1, ?2, ?3, ?4)",
            params![title, TaskStatus::Pending.as_str(), created_at, None::<String>],
        ).map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let id = self.connection.last_insert_rowid() as u32;
        Ok(id)
    }


    pub fn read_all_tasks(&self) -> AppResult<Vec<Task>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, status, created_at, completed_at FROM tasks ORDER BY id DESC"
        ).map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let tasks = stmt.query_map([], |row| {
            let status_str: String = row.get(2)?;
            let status = status_str.parse::<TaskStatus>()
                .unwrap_or(TaskStatus::Pending);

            Ok(Task::new(
                row.get(0)?,
                row.get(1)?,
                status,
                row.get(3)?,
                row.get(4)?,
            ))
        }).map_err(|e| AppError::DatabaseError(e.to_string()))?
        .collect::<SqliteResult<Vec<_>>>()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(tasks)
    }

    /// Actualiza el título de una tarea
    pub fn update_task(&mut self, id: u32, new_title: &str) -> AppResult<()> {
        if new_title.trim().is_empty() {
            return Err(AppError::ValidationError(
                "El título de la tarea no puede estar vacío".to_string(),
            ));
        }

        let rows_affected = self.connection.execute(
            "UPDATE tasks SET title = ?1 WHERE id = ?2",
            params![new_title, id],
        ).map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if rows_affected == 0 {
            return Err(AppError::TaskNotFound(id));
        }

        Ok(())
    }


    pub fn update_task_status(&mut self, id: u32, new_status: TaskStatus) -> AppResult<()> {
        let completed_at = if new_status == TaskStatus::Completed {
            Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
        } else {
            None
        };

        let rows_affected = self.connection.execute(
            "UPDATE tasks SET status = ?1, completed_at = ?2 WHERE id = ?3",
            params![new_status.as_str(), completed_at, id],
        ).map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if rows_affected == 0 {
            return Err(AppError::TaskNotFound(id));
        }

        Ok(())
    }

    /// Elimina una tarea
    pub fn delete_task(&mut self, id: u32) -> AppResult<()> {
        let rows_affected = self.connection.execute(
            "DELETE FROM tasks WHERE id = ?1",
            params![id],
        ).map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if rows_affected == 0 {
            return Err(AppError::TaskNotFound(id));
        }

        Ok(())
    }
}

