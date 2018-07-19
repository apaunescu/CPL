//showcasing Testing, I struggled significantly with getting Rust to actually run any of my imported functions.
//I tried grabbing functions from my main file, as well as my functions file, but struggled a lot.

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(5 * 2, 10);
    }

    #[test]
    fn second_test() {
        assert_eq!(1 + 2 + 3, 6);
    }
    #[test]
    fn third_test() {
        assert_eq!(1 + 2 + 3, 10);
    }
    #[test]
    fn fourth_test() {
        panic!("Failure");
    }

}

