use todo_lib as lib;

fn main() {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
    let dao = lib::Dao::new(format!("{home_dir}/.ftodo/").as_str());
    println!("Welcome to the Todo CLI!");
    println!("This is a simple command line interface for managing your tasks.");
    println!("You can add, remove, and list your tasks.");

    print_options();

    loop {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command = command.trim();

        match command {
            "list" => list_tasks(
                &dao,
                lib::QueryTodo {
                    incomplete_tasks_only: true,
                },
            ),
            "list-finished" => list_tasks(
                &dao,
                lib::QueryTodo {
                    incomplete_tasks_only: false,
                },
            ),
            "add" => add_task(&dao),
            "finish" => finish_task(&dao),
            "delete" => delete_task(&dao),
            "reset" => reset_db(&dao),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command: {}", command),
        }

        print_options();
    }
}

pub fn list_tasks(dao: &lib::Dao, query_object: lib::QueryTodo) {
    println!(
        "{} Tasks:",
        if query_object.incomplete_tasks_only {
            "Incomplete"
        } else {
            "Complete"
        }
    );

    dao.get_tasks(query_object)
        .iter()
        .for_each(|task| println!("{}: {}", task.id, task.description));
}

pub fn add_task(dao: &lib::Dao) {
    let task = get_task_from_cli();

    let added = dao.add_task(task);

    if added {
        list_tasks(
            dao,
            lib::QueryTodo {
                incomplete_tasks_only: true,
            },
        );
    } else {
        println!("Failed to add task")
    }
}

pub fn finish_task(dao: &lib::Dao) {
    list_tasks(
        dao,
        lib::QueryTodo {
            incomplete_tasks_only: true,
        },
    );

    let id = get_id_from_cli();
    let updated = dao.finish_task(id);

    if updated {
        list_tasks(
            dao,
            lib::QueryTodo {
                incomplete_tasks_only: false,
            },
        );
    } else {
        println!("Failed to finish task");
    }
}

pub fn delete_task(dao: &lib::Dao) {
    list_tasks(
        dao,
        lib::QueryTodo {
            incomplete_tasks_only: true,
        },
    );
    list_tasks(
        dao,
        lib::QueryTodo {
            incomplete_tasks_only: false,
        },
    );

    let id = get_id_from_cli();
    let deleted = dao.delete_task(id);

    if deleted {
        list_tasks(
            dao,
            lib::QueryTodo {
                incomplete_tasks_only: true,
            },
        );
        list_tasks(
            dao,
            lib::QueryTodo {
                incomplete_tasks_only: false,
            },
        );
    } else {
        println!("Failed to delete task");
    }
}

pub fn reset_db(dao: &lib::Dao) {
    let reset = dao.reset_db();

    if reset {
        println!("All tasks deleted!\n\n\n");
    } else {
        println!("Failed to reset DB")
    }
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

pub fn print_options() {
    println!("Please enter a command (list, list-finished, add, finish, delete, reset, exit): ");
}
