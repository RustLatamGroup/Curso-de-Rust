fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}