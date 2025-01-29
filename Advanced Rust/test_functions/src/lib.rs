fn is_even(number: i32) -> bool {
    match number % 2 {
        0 => true, // number is even
        _ => false // number is odd
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_is_even() {
//         assert!(is_even(42));
//     }
// }



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "should always panic!")]
    fn it_fails() {
        let result = 2 - 2;
        assert_eq!(result, 4);
    }
}
