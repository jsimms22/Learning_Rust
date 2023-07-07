/*struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Using tuple structs without named fields to create different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Struct (chpt 10 for more info)
struct AlwaysEqual;

// Constructor of sorts for User
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// If user is not passed by ref it will cause move/owner issues
// this is not inherently 'bad code', but will cause issues unless
// User contains the copy trait since user3 uses struct update syntax 
fn print_user_info(user: &User) {
    println!("{}'s account information:", user.username);
    println!("Acount email is: {}",user.email);
    println!("Account status is: {}",user.active);

    if user.sign_in_count == 1 {
        println!("User has signed in {} time",user.sign_in_count);
    } else {
        println!("User has signed in {} times",user.sign_in_count);
    }

    println!("");
}

fn main() {
    // The fields cannot individually declared 'mut' vs 'immut'
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    // Following line is a work in progress - ownership may be causing issues here
    print_user_info(&user1); 

    // How to change the value in the email field of a mutable User instance
    user1.email = String::from("anotheremail@example.com");
    
    println!("User 1's email has changed to: {}", user1.email);
    println!("");

    // using the fn build_user to declare a new "user" that is User-Type
    let /*mut*/ user2 = build_user(
        String::from("someone2@example.com"),
        String::from("someuser2name123"),
    );

    print_user_info(&user2);

    // using struct update syntax via .. to create another "user"
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    print_user_info(&user3);

    /*------------------------------------- */
    /*-----------Tuples & Struct----------- */
    /*------------------------------------- */

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    /*------------------------------------- */
    /*----------Unit-Like Struct----------- */
    /*------------------------------------- */

    let _subject = AlwaysEqual;
}*/

    /*------------------------------------- */
    /*---------------OUTPUT---------------- */
    /*------------------------------------- */

/*someone123's account information:
Acount email is: someone@example.com
User has signed in 1 time

User 1's email has changed to: anotheremail@example.com

someuser2name123's account information:
Acount email is: someone2@example.com
Account status is: true
User has signed in 1 time

someone123's account information:
Acount email is: another@example.com
Account status is: true
User has signed in 1 time*/