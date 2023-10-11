use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes

    let s1 = String::new(); // empty String
    let s2 = "initial contents"; // String slice
    let s3 = s2.to_string(); // converting a slice to an owned String
    let s4 = String::from("initial contents"); // non-empty String

    // Strings can be modified similarly to vecs

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push('!');
    println!("{}", s5); // "foobar!"

    // We can also append String using operators

    let s6 = String::from("Hello, ");
    let s7 = String::from("World!");

    // Here we are moving ownership of s6 to s8, then taking a reference of 
    // all the characters in s7 and appending them to s8
    // let s8 = s6 + &s7;
    // println!("{}", s8); // "Hello, World!"
    // We can no longer allows borrows of s6
    // We can consider using he format! macro instead to avoid ownership changes
    let s8 = format!("{}{}", s6, s7);
    println!("{}", s6);

    // Indexing Strings
    let hello = String::from("Hello");
    // We cannot use integers to index Strings since they are UTF and a collection of bytes
    // So integers would only give us a partial character sicne Rust doesn't know if we want:
    // Bytes, Scalar Values, or Grapheme Clusters
    // let c = hello[0];

    for c in hello.bytes() { // .bytes() returns us a collection of bytes
        println!("{}", c);  // c iterates over the collection of bytes and prints here
                            // 72 101 108 108 111
    }

    for c in hello.chars() { // .chars() returns us a collection of scalar/char values
        println!("{}", c); // H e l l o - of type char
    }

    // returning a collection of graphmeme clusters is not included in the standard library
    // use: unicode-segmentation crate
    for c in hello.graphemes(true) {
        println!("{}", c); // H e l l o of type &str
    }

}
