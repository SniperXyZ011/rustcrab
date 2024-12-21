fn main() {
    let s = String::from("Hello, Ownership!");

    take_ownership(s);

    // Uncommenting the line below will cause an error, as `s` is no longer valid.
    // println!("{}", s);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
