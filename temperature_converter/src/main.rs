use std::io;

fn main() {
    println!("Temperature converter");

    let mut option;

    loop {
        println!("Input F or C, to enter Fahrenheit or Celsius temperature, respectively.");
        option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line.");
        option = option.trim().to_string();
        println!("You entered: {}", option);

        if option == "F" || option == "C" {
            break;
        } else {
            continue;
        }
    }

    loop {
        println!("Enter temperature to convert from.");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line.");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(flt) => flt,
            Err(_) => continue,
        };
        println!("You entered: {}", temperature);

        if option == "F" {
            println!("Fahrenheit temperature will be converted to Celsius.");
            println!("{}°F in Celsius is {}°C.", temperature, convert_to_celsius(temperature));
            break;
        } else if option == "C" {
            println!("Celsius temperature will be converted to Fahrenheit.");
            println!("{}°C in Fahrenheit is {}°F.", temperature, convert_to_fahrenheit(temperature));
            break;
        }
    }
}

fn convert_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}

fn convert_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}