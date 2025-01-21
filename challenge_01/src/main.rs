fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // Casting variable types to Float64 bit, divide by 3.0

    let average = (a as f64
        + b /* default type is f64 so no need to cast this type */
        + c as f64)
        / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");

    let garage = [[[0; 100]; 20]; 5];

    println!("Parking number: {}", garage[1][5][2]);
    println!("On floor: {}", garage[1][5][1]);
    println!("Spot: ")
}
