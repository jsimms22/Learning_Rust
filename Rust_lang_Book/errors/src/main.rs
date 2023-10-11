// cmd = "set BACK_TRACE=1"
// cmd = "set BACK_TRACE=full" 

// $env:RUST_BACKTRACE=1
// $env:RUST_BACKTRACE="full"

// $env:RUST_BACKTRACE=1; cargo run

/* --- Unrecoverable Error Handling ---
fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(21)
}

fn c (num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}
*/

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem with creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem with opening the file: {:?}", other_error);
            },
        },
    };
}