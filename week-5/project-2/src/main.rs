use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter you age:  ");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let age:i32 = input1.trim().parse().expect("Wrong integer");

    println!("How many years of experience do you have? :  ");
    io::stdin().read_line(&mut input2).expect("Invalid string");
    let experience:i32 = input2.trim().parse().expect("Wrong integer");

    if experience >= 10 && age >= 40 {
        println!("Your salary is ₦1,560,000");
    } else if experience >= 10 && age >= 30 && age < 40 {
        println!("Your salary is ₦1,480,000");
    } else if experience > 10 && age < 28 {
        println!("Your salary is ₦1,300,000");
    } else {
        println!("Your salary is ₦100,000");
    }
}
