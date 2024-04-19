fn main() {
    let mut user1 = User { // make a (mutable) instance of the struct. Note: the entire thing is mutable; not by-key
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // access a value of that (mutable) instance

    let user2 = User { // example of making a new instance based off of an existing instance
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User { // another example with some short hand.. only need to describe changing keys
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    area_calculation();
    area_calculation_method();

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // another way to define simple structs
struct Point(i32, i32, i32);

struct AlwaysEqual; // even this works!

fn build_user(email: String, username: String) -> User { // example function to create instances of User
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User { // note the shorthand here, because the param name is the same as the key, we can omit calling it twice
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

/// 5.2
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_calculation() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    //println!("rect1 is {}", rect1); // this would fail! no println method for rectangle
    println!("rect1 is {:?}", rect1); // this works though, assuming we added line 61 #[derive(Debug)]!
    println!("rect1 is {:#?}", rect1); // this works too, prettier output
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/// 5.3
// implementing a method within a struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// implementing a method within a struct with the same name as one of the keys/fields
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn area_calculation_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

impl Rectangle { // make a square!
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}