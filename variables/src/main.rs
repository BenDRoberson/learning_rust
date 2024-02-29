fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("{THREE_HOURS_IN_SECONDS}"); // this works even though it's defined "later"!

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // next
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // 1st element

    let six_point_four = x.1; // 2nd element

    let one = x.2; // etc.

    // now array
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // array explicit type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // array of 5 3's
    let a = [3; 5];
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;