use sqlite::{Connection, State};

pub struct QueryTodo {
    pub incomplete_tasks_only: bool,
}

pub fn initialize_sqlite() {
    println!("Initializing SQLite database...");
    let connection = open_connection();

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            )",
        )
        .expect("Failed to initialize db");
}

pub fn list_tasks(query_object: QueryTodo) {
    let connection = open_connection();
    let query = "SELECT * FROM tasks WHERE completed = ?";
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

    println!(
        "{} Tasks:",
        if query_object.incomplete_tasks_only {
            "Incomplete"
        } else {
            "Complete"
        }
    );
    while let Ok(State::Row) = statement.next() {
        let task_id = statement.read::<i64, _>("id").unwrap();
        let task = statement.read::<String, _>("description").unwrap();
        println!("{}: {}", task_id, task);
    }
}

pub fn add_task() {
    let task = get_task_from_cli();

    let connection = open_connection();
    let query = "INSERT INTO tasks (description) VALUES (?)";
    let mut statement = connection
        .prepare(query)
        .expect("Insert query malformatted");
    statement
        .bind((1, task.as_str()))
        .expect("Failed to bind query");

    statement.next().expect("Failed to save todo");

    list_tasks(QueryTodo {
        incomplete_tasks_only: true,
    });
}

pub fn finish_task() {
    list_tasks(QueryTodo {
        incomplete_tasks_only: true,
    });

    let id = get_id_from_cli();

    let connection = open_connection();
    let query = "UPDATE tasks SET completed = 1 WHERE id = ?";
    let mut statement = connection
        .prepare(query)
        .expect("Update query malformatted");
    statement.bind((1, id)).expect("Failed to bind query");

    statement.next().expect("Failed to update todo");

    list_tasks(QueryTodo {
        incomplete_tasks_only: false,
    });
}

pub fn delete_task() {
    list_tasks(QueryTodo {
        incomplete_tasks_only: true,
    });
    list_tasks(QueryTodo {
        incomplete_tasks_only: false,
    });

    let id = get_id_from_cli();

    let connection = open_connection();
    let query = "DELETE FROM tasks WHERE id = ?";
    let mut statement = connection
        .prepare(query)
        .expect("Delete query malformatted");
    statement.bind((1, id)).expect("Failed to bind query");

    statement.next().expect("Failed to delete todo");

    list_tasks(QueryTodo {
        incomplete_tasks_only: true,
    });
    list_tasks(QueryTodo {
        incomplete_tasks_only: false,
    });
}

pub fn reset_db() {
    let connection = open_connection();
    let query = "DELETE FROM tasks";
    connection
        .execute(query)
        .expect("Failed to wipe db");

    println!("All tasks deleted!\n\n\n");
}

fn get_task_from_cli() -> String {
    println!("Enter your task: ");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line from terminal");

    line.trim().to_string()
}

fn get_id_from_cli() -> i64 {
    println!("Enter your task id: ");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line from terminal");

    line.trim().parse().unwrap_or(0)
}

fn open_connection() -> Connection {
    sqlite::open("todo.db").expect("Failed to open database")
}
