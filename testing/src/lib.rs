#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_doesnt_add_two() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_test() {
        let greet = greeting("Dangelo");
        assert!(greet.contains("Dangelo"));
    }

    #[test]
    fn greeting_failing_test() {
        let greet = greeting("Dangelo");
        assert!(
            !greet.contains("Shoot"),
            "Greet did not contain Shoot, values is: {}",
            greet
            );
    }


    #[test]
    #[should_panic(expected="Guess must be lower than 101")]
    fn guess_panic_test() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    #[should_panic(expected="Guess must be greater than 0")]
    fn guess_panic_test2() {
        Guess::new(-200);
    }

    // with super::* we bring the outer code into this (testing) module's scope
    #[test]
    fn private_example_test() {
        let isPrivate = private_example();
        assert!(isPrivate);
    }

}


////////////////////////////////////////////////////////////////////////////////
///////////////////////      Functions      ////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

// You can taste private functions
fn private_example() -> bool {
    true
}



////////////////////////////////////////////////////////////////////////////////
///////////////////////       Structs       ////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value <= 0 {
            panic!("Guess must be greater than 0, got {}.", value)
        } else if value > 100 {
            panic!("Guess must be lower than 101, got {}.", value)
        }
        Guess { value }
    }
}


