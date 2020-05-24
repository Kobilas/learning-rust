#![allow(dead_code)]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house;

fn serve_order() {}

mod back_of_house {
    // declaring enum as pub will also declare all variants as pub
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // declaring struct as pub will make the struct pub, but the fields will
    // remain private unless otherwise specified
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // we need a pub associated function to construct struct since at least
        // one of the fields is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;
//use self::front_of_house::hosting;
// by uncommenting one of the above lines we are able to use hosting as if
// defined in crate root scope
// rather than having to type out entire path each time

pub fn eat_at_restaurant() {
    // absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    //front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // order breakfast in summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change the bread in our order
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // next line won't compile if we uncomment
    // since seasonal_fruit not defined as pub in struct
    //meal.seasonal_fruit = String::from("Blueberries")

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
