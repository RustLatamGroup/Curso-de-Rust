fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("proccessing propellant {}...", propellant);
    propellant.push_str(" is highly fammable!");
    let length = propellant.len();
    length
}