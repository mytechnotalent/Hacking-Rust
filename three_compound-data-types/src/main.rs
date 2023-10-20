fn main() {
    let tup = (1337, 3.14, 42);
    let (x, y, z) = tup;
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");

    let x = [1, 2, 3];
    let one = x[0];
    let two = x[1];
    let three = x[2];
    println!("one: {one}");
    println!("two: {two}");
    println!("three: {three}");
}
