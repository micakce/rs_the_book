
pub fn init_vectors() {
    // let mut v: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(6);
    v.push(7);
    println!("Vector is {:?}",v);
}

pub fn vector_indexes() {

    let v = vec![1, 2, 3, 4, 5];

    // this will panic
    // let does_not_exist1 = &v[100];

    // this will give you none (and the chance of handlin the error with `match`)
    let does_not_exist2 = v.get(100);

    // println!("Values are {:?} and {:?}", does_not_exist1, does_not_exist2);

    println!("Values are {:?} and ", does_not_exist2);

}

pub fn _vector_hanldle_out_of_bound() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

pub mod stats {
    use std::collections::HashMap;


    pub fn average(data : &Vec<i32>) -> f32 {
        let mut acum = 0;
        for value in data {
            acum += value;
        }
        return acum as f32 / data.len() as f32;
    }

    pub fn median(data: &mut Vec<i32>) -> i32 {
        data.sort();
        let mid = data.len() / 2;
        return data[mid]
    }

    pub fn mode(data: &[i32]) -> HashMap<String,i32> {
        let mut counter = HashMap::new();
        let mut result = HashMap::new();
        result.insert(String::from("number"), 0);
        result.insert(String::from("ocurrences"), 0);
        for value in data {
            let count = counter.entry(value).or_insert(0);
            *count +=1;
            if *count > result["ocurrences"] {
                result.insert(String::from("number"), *value);
                result.insert(String::from("ocurrences"), *count);
            }
        }
        return result;
    }
}
