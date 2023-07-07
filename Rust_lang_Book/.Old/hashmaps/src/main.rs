fn main() {
    // by default, hashmap uses a fn called, SipHash
    // SipHash can prevent DoS attacks via hashtables
    // but sacrifices some speed
    // can create your own hasher with BuildHasher trait
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // we can access a value out of the map by providing its key to get the 'get' method
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // iterating over key/value pairs similar to vectors using a for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    /* OUTPUT *will be arbitray order
    Yellow: 50
    Blue: 10
    */

    println!("new blue key/score inserted");
    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    println!("checking is red exists with a key value, it not, inserting into map");
    // checking if a particular key has a value and if not inserting a value
    // this uses the entry method, Entry is an enum that represents a value that might or might not exist
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);

    // updating a value based on a new value
    // counts how many times eachword appears in some text
    // use a hash map with the words as keys and increment the value to keep track of how many times we've seen the word
    // if its the first time seeing the word we will insert the value 0
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // split_whitespace() iterates over sub-slices separated by whitespace of the values in text
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

    // types that impl the copy trait, i32 are copied, and strings are moves and ownership set to the map
    // field_name + field_value are no longer usable, see below:
    /*
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    */
