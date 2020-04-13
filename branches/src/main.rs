fn main() {
    let number = 3;
    // conditions must be bools
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*
    if number {
        println!("number was three");
    }
    will result in error since condition is not a bool
    correct code for this is given below
    */
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisble by 3");
    } else if number % 2 == 0 {
        println!("number is divisble by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // values must match between arms of code for example, the below code would cause an error
        // "six"
    };
    println!("The value of number is: {}", number);

    /* will execute forever until explicitly told to stop or break
    loop {
        println!("again!");
    }
    */

    // this can be used to return values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut counter = 3;
    while counter != 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    println!("LIFTOFF!!!");

    /* this code is equivalent to the for loop below the comment, though SLOWER
       this is because compiler adds runtime code to perform conditional check on elements in array in while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    */
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // countdown to liftoff code from before, but with range and .rev()
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
