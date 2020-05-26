use std::io;
/*
 * Given a list of integers, use a vector and return the mean (the average
 * value), median (when sorted, the value in the middle position), and mode (the
 * value that occurs most often; a hash map will be helpful here) of the list.
 */

fn main() {
    println!("Add numbers to vector, any non-number to stop:");
    let mut v: Vec<i32> = Vec::new();
    let mut new_num = String::new();
    loop {
        io::stdin().read_line(&mut new_num)
            .expect("Failed to read line.");
        match new_num.trim().parse::<i32>() {
            Ok(num) => {
                v.push(num);
            },
            Err(_) => {
                if v.len() >= 1 {
                    break;
                } else {
                    println!("There must be at least 1 number in vector to continue");
                }
            },
        }
        new_num = String::new();
    }
    drop(new_num);
    println!("v: {:?}", v);
}

fn vector_mean() {}

fn vector_median() {}

fn vector_mode() {}
