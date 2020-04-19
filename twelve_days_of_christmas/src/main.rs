fn main() {
    println!("Twelve Days of Christmas");
    sing_twelve_days_of_christmas();
}

fn sing_twelve_days_of_christmas() {
    let gifts = ["A partridge in a pear tree", "turtle doves, and", "french hens", "calling birds", "golden rings",
        "geese a-laying", "swans a-swimming", "maids a-milking", "ladies dancing", "lords a-leaping",
        "pipers piping", "drummers drumming"];
    for i in 1..13 {
        if i == 1 {
            println!("On the {}st day of Christmas my true love sent to me", i);
        } else if i == 2 {
            println!("On the {}nd day of Christmas my true love sent to me", i);
        } else if i == 3 {
            println!("On the {}rd day of Christmas my true love sent to me", i);
        } else {
            println!("On the {}th day of Christmas my true love sent to me", i);
        }
        for j in (0..i).rev() {
            if j == 0 {
                println!("{}", gifts[j]);
            } else {
                println!("{} {}", j+1, gifts[j]);
            }
        }
    }
}