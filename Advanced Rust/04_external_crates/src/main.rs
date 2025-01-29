use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    let num = rng.gen::<i32>();
    println!("num is {}", num);
}