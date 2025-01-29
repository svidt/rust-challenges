// use crate::greeting::formal; // absolute path
// use crate::greeting::casual;

use crate::greeting::{formal, casual}; // combining absolute path

fn main() {
   formal::english(); // hello
    casual::english(); // hi
}

mod greeting {

        pub mod formal { 

            pub fn english() {
                println!("hello");
                french();
            }

            pub fn spanish() {
                println!("hola");
            }

            pub fn french() {
                println!("bonjour");
            }
    }

    pub mod casual {

        pub fn english() {
            println!("hi");
        }

        pub fn spanish() {
            super::formal::spanish();
        }

        pub fn french() {
            println!("salut");
        }
    }
}