use std::{collections::HashMap, fmt::Display, hash::Hash, thread, time::Duration};

pub fn generate_workout<X: CValue<ItemOut=u32> + Copy + Eq + Hash >(intensity: X, random_number: u32) {

    let mut struct_clouse = Cacher::new(|num: X| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num.get()
    });

    if intensity.get() < 25 {
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

pub struct Cacher<T, U, V>
where
T: Fn(U) -> V,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
T: Fn(U) -> V,
U: Eq + Copy + Hash + CValue,
V: PartialOrd + Copy
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: U) -> V {
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

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd)]
pub struct CStr<'a> {
    value: &'a str
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd)]
pub struct CInt {
    value: u32
}

impl CInt {
    pub fn new(value: u32) -> CInt {
        CInt{ value }
    }
}


pub trait CValue {
    type Item;
    type ItemIn;
    type ItemOut : Display + PartialOrd + Copy;
    fn get(&self) -> Self::ItemOut;
}

impl<'a> CStr<'a> {
    pub fn new(value: &str) -> CStr {
        CStr{ value }
    }
}

impl<'a> CValue for CStr<'a> {
    type Item = CStr<'a>;
    type ItemIn = &'a str;
    type ItemOut = u32;

    fn get(&self) -> Self::ItemOut {
        self.value.len() as u32
    }
}

impl CValue for CInt {
    type Item = CInt;
    type ItemIn = u32;
    type ItemOut = u32;

    fn  get(&self) -> Self::ItemOut {
        self.value
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
