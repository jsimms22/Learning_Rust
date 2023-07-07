#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method for Rectangle
impl Rectangle {
    // getter fn for area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(usize: u32) -> Rectangle {
        Rectangle {
            width: usize,
            height: usize,
        }
    }
}

fn main() {
    let rect1 = Rectangle {

        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() == true {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// NOTES

    /* dbg!() */

    // println!("rect1 is {:#?}", rect1);
    // dbg! will always return ownership after
    // The macro 'dbg!' prints to the 
    // std err console stream (stderr)

    /* println! */

    // println!("rect1 is {:#?}", rect1);
    // The fn println! prints to the
    // std out console stream (stdout)

    /* replaced function code */

    /*fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
    }*/