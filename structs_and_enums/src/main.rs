mod shapes;
use shapes::rectangle::Rectangle;  // Import just the struct
mod vehicle;
use vehicle::vehicle::Vehicle;
use vehicle::vehicle::Vehicle_type;

struct Person {
    name: String,  // Fields have names and types
    age: u32,      // u32 is an unsigned 32-bit integer
    active: bool,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn main() {
    // Create an instance
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };

    // Access fields with dot notation
    println!("{} is {} years old and active: {}", person.name, person.age, person.active);

    // Structs are immutable by default, so this needs `mut`
    let mut person2 = Person {
        name: String::from("Bob"),
        age: 25,
        active: false,
    };
    println!("{} is {} years old", person2.name, person2.age);
    person2.age = 26; // Mutate a field
    println!("{} is now {} years old", person2.name, person2.age);

    let rect = Rectangle { width: 30, height: 50 }; // Use it with module prefix
    println!("Area: {}", rect.area());

    let sq = Rectangle::square(10);
    println!("Square area: {}", sq.area());

    let dir = Direction::Up;
    // Use `match` to handle variants (more on this soon)
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }

    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number"),
    }

    let carolla = Vehicle {
        make: String::from("Toyota"),
        top_speed: 90,
        vehicle_type: Vehicle_type::Car,
    };
    let frontier = Vehicle {
        make: String::from("Nissan"),
        top_speed: 85,
        vehicle_type: Vehicle_type::Truck,       
    };
    let duc = Vehicle {
        make: String::from("Ducati"),
        top_speed: 200,
        vehicle_type: Vehicle_type::Motorcycle,       
    };

    carolla.print_deets();
    frontier.print_deets();
    duc.print_deets();
}
