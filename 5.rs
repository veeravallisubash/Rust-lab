fn main() {
    let x = 42;         
    let y = 3.14;       
    let z = "Hello!";   

    println!("Implicitly typed variables:");
    println!("x = {}, y = {}, z = {}", x, y, z);

    let a: i32 = 100;       
    let b: f64 = 2.71828;
    let c: &str = "Rust!";    

    println!("\nExplicitly typed variables:");
    println!("a = {}, b = {}, c = {}", a, b, c);
}
