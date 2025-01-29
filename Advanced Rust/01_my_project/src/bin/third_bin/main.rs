// third_bind binary crate

mod another_module;

fn main() {
    println!("Running the third_bin binary crate");
    another_module::another_mod_func();
    another_module::another_mod_func();
}