use std::io;
use std::collections::HashMap;
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
    v.sort();
    println!("v: {:?}", v);
    if v.len() == 1 {
        println!("mean, median, mode of v: {}", v[0]);
    } else {
        let mean = vector_mean(&v);
        println!("mean of v: {}", mean);
        let median = vector_median(&v);
        println!("median of v: {}", median);
        let mode = vector_mode(&v);
        println!("mode of v: {}", mode);
    }
}

fn vector_mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for el in v {
        sum += el;
    }
    (sum as f64) / (v.len() as f64)
}

fn vector_median(v: &Vec<i32>) -> f64 {
    let mid: usize = v.len() / 2;
    if mid % 2 == 1 {
        ((v[mid - 1] as f64) + (v[mid] as f64)) / 2.0
    } else {
        v[mid] as f64
    }
}

fn vector_mode(v: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for el in v {
        // checks for entry el in HashMap
        // if it exists, return mutable reference to value (&mut V)
        // if it does not exist, sets key to value 0 then returns mut ref
        let count = counts.entry(*el).or_insert(0);
        // dereference count and add one
        // *&mut V -> mut V
        *count += 1;
    }
    // the above loop makes my HashMap<&i32, usize>
    // not <i32, usize> like I originally thought WOOPS
    let mut highest_key = 0;
    let mut highest_cnt = 0;
    for (key, value) in counts {
        if value > highest_cnt {
            highest_key = key;
            highest_cnt = value;
        }
    }
    highest_key
}
