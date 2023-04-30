//Program - Intermediate Program
//IT 327 - Group 4
//Names: Garett Sheley, Cameron Crone, Nate Rardin, and Quinn Pulley

//This is a function from Rust's standard library that allows for input and output
use std::env;

//Main function
fn main() {
    //Depending on what input the user gives when running the code, it will determine which vehicle's data is printed out
    let args: Vec<String> = env::args().collect();
    //Storing the user's input
    let vehicle_type = args[1].clone();
    println!("-------------------------");
    println!("Vehicle Number Chosen: {}", vehicle_type);
    //An input of "1" equates to a sedan
    if vehicle_type == "1" {
        sedan();
    }
    //An input of "2" equates to a suv
    else if vehicle_type == "2" {
        suv();
    }
    //An input of "3" equates to a sports car
    else if vehicle_type == "3" {
        sports_car();
    }
    //Any other input is invalid, and it notifies the user
    else {
        println!("Invalid Input Type");
    }
}

//The sedan function
fn sedan() {
    //Creating a instance of the Exterior structure for the sedan
    let sedan_exterior = Exterior {
        exterior_cost: 20512,
        color: String::from("Black"),
        wheels: String::from("Multispoke"),
        tires: String::from("Goodyear All-Season"),
    };
    //Creating a instance of the Interior structure for the sedan
    let sedan_interior = Interior {
        interior_cost: 1425,
        seat_material: String::from("Cloth"),
        interior_primary_color: String::from("Cream"),
        interior_trim: String::from("Birch"),
    };
    //Creating a vector (how Rust does lists) for the options for the sedan
    let options = vec!["Apple Carplay", "Heated Seats", "Adaptive Cruise Control"];
    let options_cost = 1523;
    
    //Printing out the data
    println!("-------------------------");
    println!("Vehicle Choice: Sedan");
    println!("-------------------------");
    //Passing a reference to the sedan's exterior structure
    print_exterior(&sedan_exterior);
    //Passing a reference to the sedan's interior structure
    print_interior(&sedan_interior);
    //Though it looks like this is Pass by-value, vectors are passed by reference naturally
    print_options(options);
    println!("Cost of Options: ${}", options_cost);
    println!("-------------------------");
    println!("Total Cost: ${}", (sedan_exterior.exterior_cost + sedan_interior.interior_cost + options_cost));
    println!("-------------------------");
}
//The suv function
fn suv() {
    //Creating a instance of the Exterior structure for the suv
    let suv_exterior = Exterior {
        exterior_cost: 34987,
        color: String::from("White"),
        wheels: String::from("Forged"),
        tires: String::from("BFG"),
    };
    //Creating a instance of the Interior structure for the suv
    let suv_interior = Interior {
        interior_cost: 1926,
        seat_material: String::from("Leather"),
        interior_primary_color: String::from("Black"),
        interior_trim: String::from("Aluminum"),
    };
    //Creating a vector (how Rust does lists) for the options for the suv
    let options = vec!["Apple Carplay", "Air Suspension", "Power Tailgate"];
    let options_cost = 2223;
    
    //Printing out the data
    println!("-------------------------");
    println!("Vehicle Choice: SUV");
    println!("-------------------------");
    //Passing a reference to the suv's exterior structure
    print_exterior(&suv_exterior);
    //Passing a reference to the suv's interior structure
    print_interior(&suv_interior);
    //Though it looks like this is Pass by-value, vectors are passed by reference naturally
    print_options(options);
    println!("Cost of Options: ${}", options_cost);
    println!("-------------------------");
    println!("Total Cost: ${}", (suv_exterior.exterior_cost + suv_interior.interior_cost + options_cost));
    println!("-------------------------");
}

//The sports_car function
fn sports_car() {
    //Creating a instance of the Exterior structure for the sports car
    let sports_car_exterior = Exterior {
        exterior_cost: 32147,
        color: String::from("Red"),
        wheels: String::from("Forged Aluminum"),
        tires: String::from("Pirelli p0"),
    };
    //Creating a instance of the Interior structure for the sports car
    let sports_car_interior = Interior {
        interior_cost: 3627,
        seat_material: String::from("Leather"),
        interior_primary_color: String::from("Black"),
        interior_trim: String::from("Carbon Fiber"),
    };
    //Creating a vector (how Rust does lists) for the options for the sports car
    let options = vec!["Apple Carplay", "Launch Control", "Adaptive Cruise Control", "Carbon Fiber Package", "Adaptive Rear Wing"];
    let options_cost = 4256;
    
    //Printing out the data
    println!("-------------------------");
    println!("Vehicle Choice: Sports Car");
    println!("-------------------------");
    //Passing a reference to the sports car's exterior structure
    print_exterior(&sports_car_exterior);
    //Passing a reference to the sports car's interior structure
    print_interior(&sports_car_interior);
    //Though it looks like this is Pass by-value, vectors are passed by reference naturally
    print_options(options);
    println!("Cost of Options: ${}", options_cost);
    println!("-------------------------");
    println!("Total Cost: ${}", (sports_car_exterior.exterior_cost + sports_car_interior.interior_cost + options_cost));
    println!("-------------------------");
}

//A function to print the data generated from the Exterior structure
fn print_exterior(exterior: &Exterior) {
    println!("Exterior Vehicle Configurations: ");
    println!("-------------------------");
    println!("Color: {}", exterior.color);
    println!("Wheels: {}", exterior.wheels);
    println!("Tires: {}", exterior.tires);
    println!("Total Exterior Cost: ${}", exterior.exterior_cost);
    println!("-------------------------");
}

//A function to print the data generated from the Interior structure
fn print_interior(interior: &Interior) {
    println!("Interior Vehicle Configurations: ");
    println!("-------------------------");
    println!("Seat Material: {}", interior.seat_material);
    println!("Primary Interior Color: {}", interior.interior_primary_color);
    println!("Interior Trim: {}", interior.interior_trim);
    println!("Total Interior Cost: ${}", interior.interior_cost);
    println!("-------------------------");
}

//A function that goes through and prints each element in the options vector
fn print_options(options: Vec<&str>) {
    println!("Options: ");
    println!("-------------------------");
    //Similar to a for-each loop
    for elem in options {
        print!("{}, ", elem);
    }
    println!("");
}

//The Exterior structure that can be implemented for each vehicle
struct Exterior {
    //unsigned 32 bit digit (typical for integers)
    exterior_cost: u32,
    color: String,
    wheels: String,
    tires: String,
}

//The Interior structure that can be implemented for each vehicle
struct Interior {
    //unsigned 32 bit digit (typical for integers)
    interior_cost: u32,
    seat_material: String,
    interior_primary_color: String,
    interior_trim: String,
}
