fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let slice_a = &arr[1..3];
    println!("Slice a (2nd and 3rd elements): {:?}", slice_a);

    let slice_b = &arr[..5];
    println!("Slice b (start omitted): {:?}", slice_b);

    let slice_c = &arr[5..];
    println!("Slice c (end omitted): {:?}", slice_c);

    let slice_d = &arr[..];
    println!("Slice d (start and end omitted): {:?}", slice_d);
}
