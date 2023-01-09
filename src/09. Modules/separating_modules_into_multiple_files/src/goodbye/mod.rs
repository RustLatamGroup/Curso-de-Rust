pub fn description() {
    println!("goodbye messages");
}

pub mod formal {// inline submodule
    pub fn english() {
        println!("goodbye");
    }

    pub fn spanish() {
        println!("adiós");
    }
}

pub mod casual; //submodule in same directory