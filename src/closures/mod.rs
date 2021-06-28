use std::{collections::HashMap, hash::Hash, thread, time::Duration};

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut struct_clouse = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", struct_clouse.value(intensity));
        println!("Next, do {} situps!", struct_clouse.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                struct_clouse.value(intensity)
                );
        }
    }
}

struct Cacher<T, U, V>
where
T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
T: Fn(U) -> V,
U: Eq + Copy + Hash,
V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn call_with_different_values() {
       let mut c = Cacher::new( |a| a );

       let v1 = c.value(1);
       let v2 = c.value(2);

       assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_string_slice() {
        let mut c = Cacher::new(|a: &str| a.len());

        let v1 = c.value("five");

        assert_eq!(v1, 4);
    }

}
