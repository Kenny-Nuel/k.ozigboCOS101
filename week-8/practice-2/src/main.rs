fn main() {
    let v = vec!['C', '0', 'M', 'P', 'U', 'T', 'E', 'R'];

    let mut input1 = String::new();

    println!(" Enter and index value between 0 - 7");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    // index is the non negative value which os smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid input");

    // Getting value at given index value
    let ch: char = v[index];

    print!("{} is the character for index [{}]\n", ch, index);
}
