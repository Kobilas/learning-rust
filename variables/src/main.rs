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
    {
        // default integer type is i32
        // all number literals except byte literal allow type suffix
        // and visual separator
        // decimal number literal with visual separator and type suffix
        let _x = 98_222u32;
        // hex number literal with type suffix
        let _x = 0xffu16;
        // octal number literal
        let _x = 0o77;
        // binary number literal with visual separator
        let _x = 0b1111_0000;
        // byte literal (only usable with u8 type, no type suffix/visual separator allowed)
        let _x = b'A';
    }
    {
        // default floating-point type is f64
        let _x = 2.0; // f64
        let _y: f32 = 3.0; //f32
    }
    {
        // addition
        let _sum = 5 + 10;
        // subtraction
        let _difference = 95.5 - 4.3;
        // multiplication
        let _product = 4 * 30;
        // division
        let _quotient = 56.7 / 32.2;
        // remainder
        let _remainder = 43 % 5;
    }
    {
        // boolean type is one byte in size
        let _t = true;
        let _f: bool = false;
    }
    {
        // char type is four bytes in size
        // Unicode Scalar Value
        let _c = 'z';
        let _z = 'Z';
        let _heart_eyed_cat = '😻';
    }
    {
        // tuple type
        // variety of types into one compound type
        // fixed-length
        // optional type annotations
        let _tup: (i32, f64, u8) = (500, 6.4, 1);
        let (_x, _y, _z) = _tup;
        println!("The value of y is: {}", _y);
        let _five_hundred = _tup.0;
        let _six_point_four = _tup.1;
        let _one = _tup.2;
    }
    // prefixing variable name with _ will suppress warnings about it not being used anywhere
    // as an explanation for all the underscores
    {
        // array type
        // one type into one compound type
        // fixed-length
        let a = [1, 2, 3, 4, 5];
        // useful for allocating data on stack rather than heap
        // vector is an array, but with variable-length
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
        let a = [3; 5]; // an array of 5 length filled with 3s
        // indexing out-of-bounds on an array with rust will cause a runtime error
        // Rust will panic and exit rather than trying to access that memory anyway like in most other langs
    }
}
