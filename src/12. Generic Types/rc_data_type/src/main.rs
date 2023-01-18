#![allow(dead_code)]
use std::mem;
use std::rc::Rc;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let vehicle: Shuttle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!(
        "vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );

    let vehicle_1: Rc<Shuttle> = Rc::new(vehicle);
    println!(
        "vehicle_1 size on stack: {} bytes",
        mem::size_of_val(&vehicle_1)
    );

    println!(
        "vehicle_1 size on heap: {} bytes",
        mem::size_of_val(&*vehicle_1)
    );

    let vehicle_2: Rc<Shuttle> = Rc::clone(&vehicle_1);

    let vehicle_3: Rc<Shuttle> = vehicle_2.clone();

    println!("strong ref count on Heap: {}", Rc::strong_count(&vehicle_1));

    mem::drop(vehicle_1);

    println!("strong ref count on Heap: {}", Rc::strong_count(&vehicle_3));

    {
        //start
        let vehicle_4: Rc<Shuttle> = Rc::clone(&vehicle_2);

        //Stack address
        println!(
            "la dirección del vehicle_2 en la stack es: {:p}",
            &vehicle_2
        );

        println!(
            "la dirección del vehicle_3 en la stack es: {:p}",
            &vehicle_3
        );

        println!(
            "la dirección del vehicle_4 en la stack es: {:p}",
            &vehicle_4
        );

        //Heap address
        println!(
            "la dirección del vehicle_2 en la heap es: {:p}",
            &*vehicle_2
        );

        println!(
            "la dirección del vehicle_3 en la heap es: {:p}",
            &*vehicle_3
        );

        println!(
            "la dirección del vehicle_4 en la heap es: {:p}",
            &*vehicle_4
        );

        println!("strong ref count on Heap: {}", Rc::strong_count(&vehicle_4));
        //end
    }

    println!("strong ref count on Heap: {}", Rc::strong_count(&vehicle_3));

    println!(
        "accediendo a la data desde diferentes punteros: \n{}, \n{}",
        vehicle_3.name, vehicle_2.propellant
    )

}
