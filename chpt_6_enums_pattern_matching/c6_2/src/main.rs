fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { //note {} not required unless > 1 line
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// we can use matching with 
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// we must list out every case (for example removing 'None' above will cause compiler to yell)
// what if we had 100+ values to match? how can we avoid only calling out a few cases?
// note will fail to run/build with this code here
/*
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

// OR if we don't plan to use it
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}
 */

