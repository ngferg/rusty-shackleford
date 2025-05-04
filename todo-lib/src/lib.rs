use std::fs;

use sqlite::Connection;

pub struct QueryTodo {
    pub incomplete_tasks_only: bool,
}

pub struct Task {
    pub id: i64,
    pub description: String,
}

pub struct Dao {
    connection: Connection,
}

impl Dao {
    pub fn new(db_path: &str) -> Self {
        if !std::path::Path::new(&db_path).is_dir() {
            fs::create_dir(&db_path).expect("Unable to create db directory");
        }

        let db_path = if &db_path[db_path.len() - 1..] != "/" {
            println!("perm db");
            db_path.to_string() + "/sqlite.db"
        } else {
            println!("perm db");
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

        Dao { connection }
    }

    pub fn get_tasks(&self, query_object: QueryTodo) -> Vec<Task> {
        let query = "SELECT * FROM tasks WHERE completed = ?";
        let mut statement = self
            .connection
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

    pub fn add_task(&self, task: String) -> bool {
        let query = "INSERT INTO tasks (description) VALUES (?)";
        let mut statement = self
            .connection
            .prepare(query)
            .expect("Insert query malformatted");
        statement
            .bind((1, task.as_str()))
            .expect("Failed to bind query");

        statement.next().is_ok()
    }

    pub fn finish_task(&self, id: i64) -> bool {
        let query = "UPDATE tasks SET completed = 1 WHERE id = ?";
        let mut statement = self
            .connection
            .prepare(query)
            .expect("Update query malformatted");
        statement.bind((1, id)).expect("Failed to bind query");

        statement.next().is_ok()
    }

    pub fn delete_task(&self, id: i64) -> bool {
        let query = "DELETE FROM tasks WHERE id = ?";
        let mut statement = self
            .connection
            .prepare(query)
            .expect("Delete query malformatted");
        statement.bind((1, id)).expect("Failed to bind query");

        statement.next().is_ok()
    }

    pub fn reset_db(&self) -> bool {
        let query = "DELETE FROM tasks";
        self.connection.execute(query).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    struct TestContext {
        dao: Dao,
    }

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

        dao.add_task("test the lib".to_string());

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

        dao.add_task("test the lib".to_string());
        dao.add_task("test it again".to_string());
        dao.add_task("pet the dog".to_string());

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

        dao.add_task("test the lib".to_string());
        dao.add_task("test it again".to_string());
        dao.add_task("pet the dog".to_string());
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
    fn test_db_delete_task() {
        let dao = setup();

        dao.add_task("test the lib".to_string());
        dao.add_task("test it again".to_string());
        dao.add_task("pet the dog".to_string());
        dao.add_task("delete me too".to_string());
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
    fn test_reset_db() {
        let dao = setup();

        dao.add_task("test the lib".to_string());
        dao.add_task("test it again".to_string());
        dao.add_task("pet the dog".to_string());
        dao.add_task("delete me too".to_string());
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
