use std::{io, process};

use eight::{add_employees::add_employee_ti, pig_latim::str2pig_latim};
use nine::panico;

mod eight;
mod nine;
mod closures;

fn main() {

    // eight::init_shit();
    // eight::vectors::vector_indexes();
    // println!("Hello, world!");

    // let data = vec![1,2,3,4,5,6,7,8];
    // println!("Es promedio es: {}", eight::vectors::stats::average(&data));

    // let mut data = vec![2,4,6,8,1,3,5,7];
    // let average = eight::vectors::stats::average(&mut data);
    // println!("Es promedio es: {}", average);
    // let median = eight::vectors::stats::median(&mut data);
    // println!("La media es: {}", median);
    // let mode = eight::vectors::stats::mode(&mut data);
    // println!("La media es: {:?}", mode);

    // println!("Apple -> {}", str2pig_latim("apple"));
    // println!("Flut -> {}", str2pig_latim("flut"));
    // println!("Gopher -> {}", str2pig_latim("Hनमस्ते"));

    // // panico::panico_readable();
    // let num_list = vec![4,3,5,34];
    // let highest_num = nine::trait_bounds::largest_num(&num_list);
    // println!("The highest number is: {}", highest_num);

    // let char_list = vec!['a', 'c', 'd', 'x', 'y', 'z', 'i'];
    // let highest_char = nine::trait_bounds::largest(&char_list);
    // println!("The highest char is: {}", highest_char);

    // let char_list = vec!['a', 'c', 'd', 'x', 'y', 'z', 'i'];
    // let highest_char = nine::trait_bounds::largest_no_copy(&char_list);
    // println!("The highest char_no_copy is: {}", highest_char);

    ////////////////
    //  Closures  //
    ////////////////

    loop {

        println!("Enter intensity:\n");
        let mut data = String::new();

        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read input");

        if data.trim().to_lowercase() == "done" {
            process::exit(0);
        }

        let simulated_user_specified_value : u32 = data.trim().parse().unwrap();

        println!("Enter randomness:\n");

        let mut data2 = String::new();

        io::stdin()
            .read_line(&mut data2)
            .expect("Failed to read input");
        let simulated_random_number: u32 = data2.trim().parse().unwrap();

        closures::generate_workout(simulated_user_specified_value, simulated_random_number);
    }

}
