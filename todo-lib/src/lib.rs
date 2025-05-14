use std::{fs, sync::Mutex};

use serde::{Deserialize, Serialize};
use sqlite::Connection;

pub struct QueryTodo {
    pub incomplete_tasks_only: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
}

pub struct Dao {
    connection: Mutex<Connection>,
}

impl Dao {
    pub fn new(db_path: &str) -> Self {
        if !std::path::Path::new(&db_path).is_dir() {
            fs::create_dir(&db_path).expect("Unable to create db directory");
        }

        let db_path = if &db_path[db_path.len() - 1..] != "/" {
            db_path.to_string() + "/sqlite.db"
        } else {
            db_path.to_string() + "sqlite.db"
        };

        let connection = sqlite::open(db_path).expect("Failed to open database");
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            )",
            )
            .expect("Failed to initialize db");

        Dao {
            connection: Mutex::new(connection),
        }
    }

    pub fn get_tasks(&self, query_object: QueryTodo) -> Vec<Task> {
        let query = "SELECT * FROM tasks WHERE completed = ?";
        let connection = self.connection.lock().expect("Failed to get db connection");

        let mut statement = connection
            .prepare(query)
            .expect("Select query malformatted");
        statement
            .bind((
                1,
                if query_object.incomplete_tasks_only {
                    0
                } else {
                    1
                },
            ))
            .expect("Failed to bind query");

        statement
            .iter()
            .filter_map(|row| {
                if let Ok(row) = row {
                    Some(Task {
                        id: row.read::<i64, _>("id"),
                        description: row.read::<&str, _>("description").to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn add_task(&self, task: &str) -> Option<i64> {
        let query = "INSERT INTO tasks (description) VALUES (?)";
        let connection = self.connection.lock().expect("Failed to get db connection");
        let mut statement = connection
            .prepare(query)
            .expect("Insert query malformatted");
        statement.bind((1, task)).expect("Failed to bind query");

        let result = statement.next();

        match result {
            Ok(sqlite::State::Done) => {
                let last_id_query = "SELECT last_insert_rowid()";
                let mut last_id_statement = connection
                    .prepare(last_id_query)
                    .expect("Failed to prepare last_insert_rowid query");
                if let Ok(sqlite::State::Row) = last_id_statement.next() {
                    Some(
                        last_id_statement
                            .read::<i64, _>(0)
                            .expect("Unable to get last id"),
                    )
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn finish_task(&self, id: i64) -> bool {
        let query = "UPDATE tasks SET completed = 1 WHERE id = ?";
        let connection = self.connection.lock().expect("Failed to get db connection");
        let mut statement = connection
            .prepare(query)
            .expect("Update query malformatted");
        statement.bind((1, id)).expect("Failed to bind query");

        statement.next().is_ok()
    }

    pub fn unfinish_task(&self, id: i64) -> bool {
        let query = "UPDATE tasks SET completed = 0 WHERE id = ?";
        let connection = self.connection.lock().expect("Failed to get db connection");
        let mut statement = connection
            .prepare(query)
            .expect("Update query malformatted");
        statement.bind((1, id)).expect("Failed to bind query");

        statement.next().is_ok()
    }
    
    pub fn update_description(&self, id: i64, new_description: &str) -> bool {
        let query = "UPDATE tasks SET description = ? WHERE id = ?";
        let connection = self.connection.lock().expect("Failed to get db connection");
        let mut statement = connection
            .prepare(query)
            .expect("Update query malformatted");
        statement.bind((1, new_description)).expect("Failed to bind query");
        statement.bind((2, id)).expect("Failed to bind query");

        statement.next().is_ok()
    }

    pub fn delete_task(&self, id: i64) -> bool {
        let query = "DELETE FROM tasks WHERE id = ?";
        let connection = self.connection.lock().expect("Failed to get db connection");
        let mut statement = connection
            .prepare(query)
            .expect("Delete query malformatted");
        statement.bind((1, id)).expect("Failed to bind query");

        statement.next().is_ok()
    }

    pub fn reset_db(&self) -> bool {
        let query = "DELETE FROM tasks";
        let connection = self.connection.lock().expect("Failed to get db connection");
        connection.execute(query).is_ok()
    }
}

unsafe impl Send for Dao {}
unsafe impl Sync for Dao {}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn setup() -> Dao {
        let dao = Dao::new("target/test");
        dao.reset_db();
        dao
    }

    #[test]
    #[serial]
    fn test_empty_db_gets_empty_lists() {
        let dao = setup();

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(0, incomplete_tasks.len());
        assert_eq!(0, complete_tasks.len());
    }

    #[test]
    #[serial]
    fn test_db_one_add() {
        let dao = setup();

        dao.add_task("test the lib");

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(1, incomplete_tasks.len());
        assert_eq!(0, complete_tasks.len());
    }

    #[test]
    #[serial]
    fn test_db_three_add() {
        let dao = setup();

        let id = dao.add_task("test the lib").unwrap();
        assert_eq!(1, id);
        let id = dao.add_task("test it again").unwrap();
        assert_eq!(2, id);
        let id = dao.add_task("pet the dog").unwrap();
        assert_eq!(3, id);

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(3, incomplete_tasks.len());
        assert_eq!(0, complete_tasks.len());
    }

    #[test]
    #[serial]
    fn test_db_finish_task() {
        let dao = setup();

        dao.add_task("test the lib");
        dao.add_task("test it again");
        dao.add_task("pet the dog");
        dao.finish_task(2);
        dao.finish_task(3);

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(1, incomplete_tasks.len());
        assert_eq!(2, complete_tasks.len());
    }

    #[test]
    #[serial]
    fn test_db_unfinish_task() {
        let dao = setup();

        dao.add_task("test the lib");
        dao.add_task("test it again");
        dao.add_task("pet the dog");
        dao.finish_task(2);
        dao.finish_task(3);
        dao.unfinish_task(3);

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(2, incomplete_tasks.len());
        assert_eq!(1, complete_tasks.len());
    }

    #[test]
    #[serial]
    fn test_db_delete_task() {
        let dao = setup();

        dao.add_task("test the lib");
        dao.add_task("test it again");
        dao.add_task("pet the dog");
        dao.add_task("delete me too");
        dao.finish_task(2);
        dao.finish_task(3);
        dao.delete_task(2);
        dao.delete_task(4);

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(1, incomplete_tasks.len());
        assert_eq!(1, complete_tasks.len());
    }

    #[test]
    #[serial]
    fn test_db_update_task_des() {
        let dao = setup();

        dao.add_task("this is old");
        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        assert_eq!("this is old".to_string(), incomplete_tasks[0].description);

        let updated = dao.update_description(1, "this is new");
        assert!(updated);
        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        assert_eq!("this is new".to_string(), incomplete_tasks[0].description);
    }

    #[test]
    #[serial]
    fn test_reset_db() {
        let dao = setup();

        dao.add_task("test the lib");
        dao.add_task("test it again");
        dao.add_task("pet the dog");
        dao.add_task("delete me too");
        dao.finish_task(2);
        dao.finish_task(3);
        dao.delete_task(2);
        dao.delete_task(4);
        dao.reset_db();

        let incomplete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: true,
        });
        let complete_tasks = dao.get_tasks(QueryTodo {
            incomplete_tasks_only: false,
        });

        assert_eq!(0, incomplete_tasks.len());
        assert_eq!(0, complete_tasks.len());
    }
}
