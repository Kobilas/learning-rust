use std::io;

fn main() {
    println!("Find the nth Fibonacci number.");
    loop {
        println!("Input n, for finding the nth Fibonacci number.");
        let mut n = String::new();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line.");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You entered: {}", n);

        println!("The {}th Fibonacci number is {}", n, get_nth_fibonacci_number(n));
        break;
    }
}

fn get_nth_fibonacci_number(n: u32) -> u32 {
    let mut n1 = 0;
    let mut n2 = 1;
    if n == 0 {
        return n1;
    } else if n == 1 {
        return n2;
    }
    let mut counter = 1;
    let mut xn = n1 + n2;
    while counter != n {
        counter += 1;
        n2 = n1;
        n1 = xn;
        xn = n1 + n2;
    }
    xn
}