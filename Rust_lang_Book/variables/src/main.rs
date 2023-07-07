fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
       
    //can redeclare the variable to change data without mut
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);
       
    const _COUNT: u32 = 100000; //convention to capitalize constants

    /*SCALAR DATATYPES
    ->Integers
    ->Floating-Point Numbers
    ->Booleans
    ->Char */

    /* INTEGERS */
    let a = 98_222; //decimal, default is i32
    let b = 0xff; //hex
    let c = 0o77; //octal
    let d = 0b1111_0000; //binary
    let e = b'A'; //byte
    let f: u8 = 255; //256 becomes 0, 257 becomes 1, and so on

    /* FLOATING-POINT */
    let f = 2.0; //default is f64
    let g: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    /* BOOLEANS */
    let t = true;
    let f= false;

    /* CHARACTER */
    let c = 'z';

    /* COMPOUND TYPES */
    let tup = ("Let's learn Rust!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1; //tup.0 is &str

    let error_codes = [100, 404, 500]; //arrays start at index 0
    let not_found = error_codes[1];

    let byte = [0;8]; //sets byte equal to an array of 8 indices all equal to 0

    let sum = my_function(11, 12);
    println!("sum = {}", sum);

    /* CONTROL FLOW*/
    let number = 10;

    //must be explicitly a boolean
    if number > 10 {
        println!("first condition was true.");
    } else if number < 10 {
        println!("second condition was true.");
    } else {
        println!("third condition was true.");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number now equals = {}", number);

    /* CONTROL FLOW CONT. - LOOPS */
    // loop
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("Loop counter result is {}", counter);

    //while
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("While Loop complete.");

    //for

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for n in 1..4 { //range, prints 1 to 3
        println!("{}!", n);
    }

}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of input x is: {}", x);
    println!("The value of input y is: {}", y);
    //let sum = x + y;
    x + y
    //or
    // return sum;
    //or
    // sum
}
