fn main() {
    let x = 5; // Immutable variable
    println!("x = {}", x);

    let mut y = 10; // Mutable variable
    println!("y before mutation = {}", y);
    y = 15;
    println!("y after mutation = {}", y);
}
