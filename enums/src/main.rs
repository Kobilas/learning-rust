#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama, Alaska, Arizona, Arkansas, California,
    Colorado, Connecticut, Delaware, Florida, Georgia,
    Hawaii, Idaho, Illinois, Indiana, Iowa,
    Kansas, Kentucky, Louisiana, Maine, Maryland,
    Massachusetts, Michigan, Minnesota, Mississippi, Missouri,
    Montana, Nebraska, Nevada, NewHampshire, NewJersey,
    NewMexico, NewYork, NorthCarolina, NorthDakota, Ohio,
    Oklahoma, Oregon, Pennsylvania, RhodeIsland, SouthCarolina,
    SouthDakota, Tennessee, Texas, Utah, Vermont,
    Virginia, Washington, WestVirginia, Wisconsin, Wyoming,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let lucky_penny = Coin::Penny;
    value_in_cents(lucky_penny);
    let coin_to_try = Coin::Quarter(UsState::NewJersey);
    value_in_cents(coin_to_try);

    let five = Some(5);
    println!("five: {:?}", five);
    let six = plus_one(five);
    println!("six: {:?}", six);
    println!("five, after plus_one: {:?}", five);
    let none = plus_one(None);
    println!("none: {:?}", none);

    let some_u8_value = Some(0u8);
    println!("some_u8_value: {:?}", some_u8_value);
    test_match_if_let(some_u8_value);
    let some_u8_three = Some(3u8);
    println!("some_u8_three: {:?}", some_u8_three);
    test_match_if_let(some_u8_three);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_match_if_let(some_u8_value: Option<u8>) {
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        ()
    }
}