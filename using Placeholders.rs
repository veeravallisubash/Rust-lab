fn main() {
    for i in 1..=5 {
        let output = format!("{}", i.to_string().repeat(i));
        println!("{}", output);
    }
}
