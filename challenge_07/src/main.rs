struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width,
            height: height
        }
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f64) {
        let scaled_width = self.width * factor;
        let scaled_height = self.height * factor;
        
        self.width = scaled_width;
        self.height = scaled_height;
    }
}


fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!")

}