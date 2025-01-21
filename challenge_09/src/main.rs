use std::fmt;


struct Satellite {
    name: String,
    velocity: f64
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Satellite Struct\nname: {}\nvelocity: {}", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("{}", hubble);
}
