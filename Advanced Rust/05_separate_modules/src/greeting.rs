pub fn description() {
    println!("greeting messages")
}

pub mod formal { // inline submodule
    pub fn english() {
        println!("hello");
    }

    pub fn spanish() {
        println!("hola");
    }
}

pub mod casual; // submodule in greeting/ directory