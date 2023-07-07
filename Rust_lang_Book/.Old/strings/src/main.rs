fn main() {
    let mut s = String::new();

    let data = "initial contents";

    s = data.to_string();

    // the method also works on a literal directly:
    s = "initial contents".to_string();

    // equivilant to the to_string() method
    s = String::from("initial contents");

    // Strings have methods similar to Vec<T>
    s = String::from("foo");
    s.push_str("bar");
    // the above code does not take ownership
    // the following push.str() takes owernship of s2
    // therefore the println doesn't work as expected
    /*
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    */

    // push() takes a single char and pushes the input to string s
    s = String::from("lo");
    s.push('l');

    // string concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // The reason s1 is no longer valid after the addition, and the reason we used 
    // a reference to s2, has to do with the signature of the method that’s called 
    // when we use the + operator.

    // The + operator uses the add method, whose signature looks something like this:
    /* fn add(self, s: &str) -> String { */

    // concatenation mulitple items
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    // s = s4 + "-" + &s5 + "-" + &s6;
    // for more complicated string concatenation use the format! macro
    // format! is a micro similar to println! but returns string to variable not console
    s = format!("{}-{}-{}", s4, s5, s6);
    // format! macro uses references so that this call doesn’t take ownership of any of its parameters.

    // slicing in rust
    let hello = "Здравствуйте";
    // this is saying I wasn bytes 0 through 4
    let s_c = &hello[0..4];

    // best way is to distinguish if you want explicitly bytes or chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

