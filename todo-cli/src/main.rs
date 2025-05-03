mod dao;

fn main() {
    dao::initialize_sqlite();
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
            "list" => dao::list_tasks(dao::QueryTodo {
                incomplete_tasks_only: true,
            }),
            "list-finished" => dao::list_tasks(dao::QueryTodo {
                incomplete_tasks_only: false,
            }),
            "add" => dao::add_task(),
            "finish" => dao::finish_task(),
            "delete" => dao::delete_task(),
            "reset" => dao::reset_db(),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command: {}", command),
        }

        print_options();
    }
}

fn print_options() {
    println!("Please enter a command (list, list-finished, add, finish, delete, reset, exit): ");
}
