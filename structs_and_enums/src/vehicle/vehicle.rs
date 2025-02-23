pub struct Vehicle {
    pub make: String,
    pub top_speed: i32,
    pub vehicle_type: Vehicle_type
}

impl Vehicle {
    pub fn print_deets(&self) {
        println!("I'm a {} that can go {} mph", self.make, self.top_speed);
        match self.vehicle_type {
            Vehicle_type::Car => println!("I'm a car!"),
            Vehicle_type::Truck => println!("I'm a truck!"),
            Vehicle_type::Motorcycle => println!("I'm a motorcycle!"),
        }
    }
}

pub enum Vehicle_type {
    Car,
    Truck,
    Motorcycle,
}
