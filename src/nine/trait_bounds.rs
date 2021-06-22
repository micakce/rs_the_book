pub fn largest_num(list : &[i32]) -> i32 {
    let mut highest = list[0];

    for &number in list {
        if number > highest {
            highest = number;
        }
    }
    return highest;
}

// pub fn largest_1<T>(list : &[T]) -> T {
//     let mut highest = list[0];
//     for &number in list {
//         if number > highest {
//             highest = number;
//         }
//     }
//     return highest;
// }

pub fn largest<T: PartialOrd+Copy>(list : &[T]) -> T {
    let mut highest = list[0];
    for &number in list {
        if number > highest {
            highest = number;
        }
    }
    return highest;
}

pub fn largest_no_copy<T: PartialOrd>(list : &[T]) -> &T {
    let mut highest = &list[0];
    for number in list {
        if number > highest {
            highest = number;
        }
    }
    return highest;
}
