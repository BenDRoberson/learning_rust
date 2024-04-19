fn main() {
    passing_reference();
    passing_mutable(); //this would succeed b/c you can change a mutable string
    //let reference_to_nothing = dangle(); // if you try and run this it will fail; rust won't allow you to create a dangling reference
    let _reference_to_something = no_dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn passing_reference(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn passing_mutable(){
    let mut s = String::from("hello");

    change(&mut s);

    println!("The value of s is {s}")
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* fn dangle() -> &String {
    let s = String::from("hello");

    &s
} */

fn no_dangle() -> String {
    let s = String::from("hello");

    s // works because it returns the string directly; rather than the reference
}