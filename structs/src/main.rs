// annontation allows for pretty-printing of this struct in Debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // borrowing self immutably since we just want to read from it
    // can also do &mut self to borrow self mutably if we did want to change/write to it
    // can ALSO take ownership of self by just using self as a parameter
    // this is rare and usually only the case for if we want to change the reference
    // to make sure it is no longer usable after the method call
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // using two variables
    let width = 30;
    let height = 50;
    println!("The area of rectangle is {} square pixels.", area(width, height));

    // using one tuple
    let rect1 = (30, 50);
    println!("The area of rectangle is {} square pixels.", area_w_tuple(rect1));

    // using one struct
    // using Field Init Shorthand to initialize struct since the attributes of
    // struct and the values we are assigning have the same name cool
    let rect2 = Rectangle {
        width,
        height
    };
    // pretty print of rect struct
    println!("rect2 is (using :? pretty-print syntax) {:?}", rect2);
    println!("rect2 is (using :#? pretty-print syntax) {:#?}", rect2);
    println!("The area of rectangle is {} square pixels.", area_w_struct(&rect2));

    // using one struct that has a method for getting the area
    println!("The area of rectangle is {} square pixels.", rect2.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_w_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_w_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}