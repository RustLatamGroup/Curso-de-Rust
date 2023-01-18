#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64
}

fn main() {
    let hubble = Satellite {
        name: String::from("hubble Telescope"),
        velocity: 4.72
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}
