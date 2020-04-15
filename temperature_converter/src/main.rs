use std::io;

fn main() {
    println!("Temperature converter");

    let mut option = String::new();

    loop {
        println!("Input F or C, to enter Fahrenheit or Celsius temperature, respectively.");
        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line.");
        let option = option.trim();
        println!("You entered: {}", option);

        if option == "F" {
            println!("Fahrenheit temperature will be converted to Celsius.");
            break;
        } else if option == "C" {
            println!("Celsius temperature will be converted to Fahrenheit.");
            break;
        } else {
            continue;
        }
    }
}

/*
fn convert_to_fahrenheit(temperature: i32) -> i32 {

}
*/