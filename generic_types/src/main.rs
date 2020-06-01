#![allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

// defining generic type T for impl so that we can impl on Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// impl on Point with static type <f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwo<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointTwo<T, U> {
    fn mixup<V, W> (self, other: PointTwo<V, W>) -> PointTwo<T, W> {
        PointTwo {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number using largest_i32 is {}", result);
    let result = largest(&number_list);
    println!("The largest number using largest is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char using largest_char is {}", result);
    let result = largest(&number_list);
    println!("The largest char using largest is {}", result);
    
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    
    let p1 = PointTwo { x: 5, y:10.4 };
    let p2 = PointTwo { x: "Hello", y: 'c' };
    let _p3 = p1.mixup(p2);
}


// function can operate on an abstract list instead of specific types
// genericly-typed function can similarly operate on an abstract list of
// abstract type, as shown in last function below
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

// Commented out until traits are properly understood
fn  largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}