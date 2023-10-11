fn main() {
    let a = [1, 2, 3];

    // Rust cannot infer with declarations of values
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    // macro for initializing a vec
    // vecs are allocated in the heap
    // they will be removed when leaving scope
    {
        let v2 = vec![1, 2, 3];
    }

    let v3 = vec![1, 2, 3, 4, 5];

    let third = &v[2];  
    // below is a runtime error since vecs are not allocated at compile time
    // let third = &v[20];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // better to retrieve elements of a vec with get via match statement
    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];
    
    // iterating across all elements in vec
    for i in &v4 {
        println!("{}", i);
    }

    // iterating and modifying across all elements in vec
    for i in &mut v4 {
        *i += 50;
    }
    println!("{:?}", v4);

    
}
