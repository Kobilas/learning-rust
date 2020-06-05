pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!(" - {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("{} ({})", self.content, self.summarize_author())
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number using largest_i32 is {}", result);
    let result = largest(&number_list);
    println!("The largest number using largest is {}", result);
    let result = largest_clone(&number_list);
    println!("The largest number using largest_clone is {}", result);
    let result = largest_reference(&number_list);
    println!("The largest number using largest_reference is {}", *result);

    let char_list = vec!['m', 'm', 'y', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char using largest_char is {}", result);
    let result = largest(&char_list);
    println!("The largest char using largest is {}", result);
    let result = largest_clone(&char_list);
    println!("The largest char using largest_clone is {}", result);
    let result = largest_reference(&char_list);
    println!("The largest char using largest_reference is {}", *result);

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

/*
pub fn notify(item: impl Summary + Display)
pub fn notify<T: Summary + Display>(item: T)
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone
          U: Clone + Debug
*/

// Commented out until traits are properly understood
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Using Clone trait instead of Copy trait
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let my_list = list.clone();
    let mut largest = list[0].clone();
    for item in my_list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

// implement without using either Copy nor Clone
// I don't know how i did thids
fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    let mut idx = 0;
    for (i, item) in list.iter().enumerate() {
        if item > largest {
            largest = &item;
            idx = i;
        }
    }
    &list[idx]
}