use std::{collections::HashMap, hash::Hash};

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    
    let mut scores = HashMap::new();

    scores.insert(blue, 10); 
    scores.insert(yellow, 50);

    // note we are not referencing blue so ownership is transferred to the map from blue
    // passing via reference would require life times

    // println!("{}", blue); // ownership error occurs here

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // We can iterate over the whole hashmap via:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // inserts value: 10 using key: Blue
    scores.insert(String::from("Blue"), 20); // overwrites the current value to new value for key: Blue
 
    scores.entry(String::from("Yellow")).or_insert(30); 
    // inserts value: 30 using key: Yellow using an entry enum
    // then we can call methods on the enum, here we use or_insert
    // or_insert says: if there isn't a key then insert value
    // if key does exist, do nothing 
    scores.entry(String::from("Yellow")).or_insert(50);    // or_insert() does nothing here

    // Updating a hash map based on an old value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
