const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    {
        // code will cause error without mut because compiler is operating under assumption that
        // this variable will not change (due to not being specified as mutable)
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }
    {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    {
        let spaces = "   ";
        let spaces = spaces.len();
        println!("Number of spaces: {}", spaces);
    }
}
