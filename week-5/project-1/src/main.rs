use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("What is the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid integer");

    println!("What is the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid integer");

    println!("What is the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid integer");

    let root = (b.powf(2.0)) - (4.0 * a * c);
    let square_root = root.sqrt();
    let x1 = (-(b) - square_root) / (2.0 * a);
    let x2 = (-(b) + square_root) / (2.0 * a);

    if x1.is_nan() || x2.is_nan() {
        println!("The roots of the equation are not real");
    }   
    else if x1 == x2 {
        println!("The roots of the equation are {} (twice)", x1);
    } else {
        println!("The roots of the equation are x = {} or {}", x1, x2);
    }
}