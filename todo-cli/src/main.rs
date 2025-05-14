use todo_lib as lib;
use todo_lib::Dao;

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
            "unfinish" => unfinish_task(&dao),
            "update-description" => update_description(&dao),
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

pub fn list_tasks(dao: &Dao, query_object: lib::QueryTodo) {
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

pub fn add_task(dao: &Dao) {
    let task = get_task_from_cli();

    let new_task_id = dao.add_task(task.as_str());

    match new_task_id {
        Some(id) => println!("Added task with id {}", id),
        _ => println!("Failed to add task"),
    }
}

pub fn finish_task(dao: &Dao) {
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

pub fn unfinish_task(dao: &Dao) {
    list_tasks(
        dao,
        lib::QueryTodo {
            incomplete_tasks_only: false,
        },
    );

    let id = get_id_from_cli();
    let updated = dao.unfinish_task(id);

    if updated {
        list_tasks(
            dao,
            lib::QueryTodo {
                incomplete_tasks_only: true,
            },
        );
    } else {
        println!("Failed to unfinish task");
    }
}

fn update_description(dao: &Dao) {
    list_all_tasks(dao);

    let id = get_id_from_cli();
    let desc = get_task_from_cli();
    let updated = dao.update_description(id, desc.as_str());

    if updated {
        list_all_tasks(dao);
    } else {
        println!("Failed to update task description");
    }
}

pub fn delete_task(dao: &Dao) {
    list_all_tasks(dao);

    let id = get_id_from_cli();
    let deleted = dao.delete_task(id);

    if deleted {
        list_all_tasks(dao);
    } else {
        println!("Failed to delete task");
    }
}

pub fn reset_db(dao: &Dao) {
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

fn list_all_tasks(dao: &Dao) {
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
}

pub fn print_options() {
    println!(
        "Please enter a command (list, list-finished, add, finish, unfinish, update-description, delete, reset, exit): "
    );
}
