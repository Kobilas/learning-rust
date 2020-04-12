// statements are instructions that perform some action and do not return a value
// expressions evaluate to a resulting value

fn main() {
    // Rust code uses snake case as style for function and variable names
    // all letters lowercase and underscores separate words
    println!("Hello, world!");
    another_function(5, 6);

    let i = 5;

    let j = {
        let i = 3;
        // not ending the below line with semicolon makes it an EXPRESSION
        // ending lines with semicolons make them statements
        i + 1
    };

    println!("The value of i is: {}", i);
    println!("The value of j is: {}", j);

    let five_returned = five();

    println!("The value of five_returned is: {}", five_returned);

    let six = plus_one(five_returned);

    println!("The value of six is: {}", six);
}

// must annotate parameters types
// compiler will not need to use them elsewhere to figure out what you mean
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// don't name return values, but do declare type with arrow
// return value of function is synonymous with value of final expression in body of function
// can return early by using return keyword, but most functions return last expression implicitly
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}