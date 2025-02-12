fn main() {
    let rect = shape::Rectangle::new(3.0, 4.0);
    println!("Area of rectangle: {}", rect.get_area());
    println!("Width of rectangle: {}", rect.width);
}

mod shape {
    pub struct Rectangle {
        pub width: f64,
        height: f64,
    }
    
    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle {
                width,
                height
            }
        }
    
        pub fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }
}
