use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let number_of_siblings = input1.trim().parse();

    let arr = [];

    for index in number_of_siblings {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Invalid string");
        let age_of_sibling = input2.trim().parse();
        let new_age = arr[index];
    }


}
