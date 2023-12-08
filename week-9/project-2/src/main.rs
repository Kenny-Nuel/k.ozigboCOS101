use std::io::Write;
use std::io;

fn main() {
    println!("**** STUDENT MANAGEMENT INFORMATION SYSTEM****");

    // Inputs for the lager, stout and non-alcoholic drinks to be inputed
    let mut input1 = String::new();
    println!("How many students do you want to record: ");
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let number_of_students:u32 = input1.trim().parse().expect("Incorrect integer");

    let mut name: Vec<String> = Vec::new();
    let mut matric_number: Vec<String> = Vec::new();
    let mut department: Vec<String> = Vec::new();
    let mut level: Vec<String> = Vec::new();

    for _ in 0..number_of_students {
        let mut input2 = String::new();
        println!("Enter Your Full Name: ");
        io::stdin().read_line(&mut input2).expect("Invalid input");
        let student_name = input2.trim().to_string();
        name.push(student_name);

        let mut input3 = String::new();
        println!("Matric Number: ");
        io::stdin().read_line(&mut input3).expect("Invalid input");
        let matric_no = input3.trim().to_string();
        matric_number.push(matric_no);

        let mut input4 = String::new();
        println!("Department: ");
        io::stdin().read_line(&mut input4).expect("Invalid input");
        let dept = input4.trim().to_string();
        department.push(dept);

        let mut input5 = String::new();
        println!("Level: ");
        io::stdin().read_line(&mut input5).expect("Invalid input");
        let class = input5.trim().to_string();
        level.push(class)

    }

    let mut file = std::fs::File::create("data.txt").expect("Create failed");
    file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    file.write_all(
        format!("{:^80}\n", "PAU SMIS")
            .as_bytes(),
    )
    .expect("Write failed");
    file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    file.write_all(
        format!("{:^19} {:^0} {:^19} {:^0} {:^19} {:0} {:20}\n", "Student Name", "|", "Matric. Number", "|", "Department", "|", "Level")
            .as_bytes(),
    )
    .expect("Write failed");
    file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");


    for i in 0..name.len() {
        let first_item = &name[i];
        let second_item = &matric_number[i];
        let third_item = &department[i];
        let fourth_item = &level[i];

        file.write_all(format!("{:^20}",first_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",second_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",third_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("| ".as_bytes()).expect("Write failed");
        file.write_all(format!("{:^20}",fourth_item.trim()).as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("------------------------------------------------------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
    }
}
