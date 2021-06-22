use std::{collections::HashMap, io};

pub fn add_employee_ti() {

    let mut db : HashMap<String, Vec<String>>= HashMap::new();

    loop {
        println!("Enter Employee and Department");

        let mut data = String::new();

        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read input");

        let words : Vec<&str> = data.split(' ').collect();

        match words.len() {
            1 => {
                match db.get(words[0]) {
                    Some(v) => println!("{:?}", v),
                    None => println!("Wrong department"),
                }
            },
            4 => {
                let department = String::from(words[3]);
                let employee = String::from(words[1]);
                // let list = db.entry(department).or_insert(Vec::new());
                db.entry(department).or_insert(Vec::new()).push(employee);
            },
            _ => println!("Not valid input")
        }
    }
}
