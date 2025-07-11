fn main() {
    let x = 5;
    println!("Outside block: x = {}", x);

    {
        let x = x + 1;
        println!("Inside inner block: x = {}", x);

        let y = 10;
        println!("Inside inner block: y = {}", y);
    }

    println!("After block: x = {}", x);

    let x = x * 2;
    println!("After shadowing x in outer scope: x = {}", x);
}
