#[derive(Debug)]
enum Shape {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64, f64) // sides a, b, c
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn DDmain() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);

    println!("\n#######\n");

    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = match number { // Could also use .unwrap_or(0)
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {}", number);

}

enum Option<T> {
    Some(T),
    None
}

