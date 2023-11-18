use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();

    let medical_conditions = "Below are a couple of diseases we diagnose:\n
    Alzheimer (AL) \n
    Arrhythmia (AR) \n
    Chronic Kidney Disease (CKD) \n
    Diabetes (D) \n
    Arthritis (ART)";

    println!("{}", medical_conditions);

    println!("We are going to need a few details from you for the test.");
    println!("Input your name as stated in your passport");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let name = input1.trim();

    println!("Your day of birth (D)");
    io::stdin().read_line(&mut input2).expect("Invalid string");
    let day_of_birth:i32 = input2.trim().parse().expect("Invalid integer");

    println!("Your month of birth (M)");
    io::stdin().read_line(&mut input3).expect("Invalid string");
    let month_of_birth:i32 = input3.trim().parse().expect("Invalid integer");

    println!("Your year of birth (YYYY)");
    io::stdin().read_line(&mut input4).expect("Invalid string");
    let year_of_birth:i32 = input4.trim().parse().expect("Invalid integer");

    println!("Your email address: ");
    io::stdin().read_line(&mut input5).expect("Invalid string");
    let email = input5.trim();

    println!("Your phone number: ");
    io::stdin().read_line(&mut input6).expect("Invalid string");
    let phone_number = input6.trim();

    println!("Number of siblings: ");
    io::stdin().read_line(&mut input7).expect("Invalid string");
    let number_of_siblings:i32 = input7.trim().parse().expect("Invalid integer");

    println!("Number of children: ");
    io::stdin().read_line(&mut input11).expect("Invalid string");
    let number_of_children:i32 = input11.trim().parse().expect("Invalid integer");

    println!("What have you been diagnosed with ? ");
    io::stdin().read_line(&mut input8).expect("Invalid string");
    let disease = input8.trim();

    println!("Name of your village: ");
    io::stdin().read_line(&mut input9).expect("Invalid string");
    let village = input9.trim();

    println!("What year are you in? ");
    io::stdin().read_line(&mut input10).expect("Invalid string");
    let current_year:i32 =  input10.trim().parse().expect("Invalid integer");

    let age:i32 = current_year - year_of_birth;

    if (disease.to_lowercase() == "al") && (age > 50) && (number_of_children > 4) && (village == "Akpabom") {
        let bill:i32 = 1_200_000 * (20/100);
        println!("Your name is: {}\n
            Your date of birth is: {}/{}/{}\n
            Your email address is {} \n
            Your phone number is: {}\n
            You have {} sibling(s) \n 
            You have {} child(ren) \n 
            You have been diagnosed with Alzheimer({})\n 
            You are from {}\n 
            Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if (disease.to_lowercase() == "ar") && (age == 30) && (number_of_siblings > 4) && (village == "Ngbauji") {
        let bill:i32 = 550_000 * (5/100);
        println!("Your name is: {}\n
            Your date of birth is: {}/{}/{}\n
            Your email address is {} \n
            Your phone number is: {}\n
            You have {} sibling(s) \n 
            You have {} child(ren) \n 
            You have been diagnosed with Arrhythmia({})\n 
            You are from {}\n 
            Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if (disease.to_lowercase() == "ckd") && (age > 40) && (number_of_children > 3) && (number_of_siblings > 3) && (village == "Atabrikang") {
        let bill:i32 = 1_500_000 * (15/100);
        println!("Your name is: {}\n
            Your date of birth is: {}/{}/{}\n
            Your email address is {} \n
            Your phone number is: {}\n
            You have {} sibling(s) \n 
            You have {} child(ren) \n 
            You have been diagnosed with Chronic Kidny Disease({})\n 
            You are from {}\n 
            Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if (disease.to_lowercase() == "d") && (age > 28) && (age < 45) && (number_of_children > 2 ) && (number_of_children < 4)&& (village == "Okorobilom") {
        let bill:i32 = 800_000 * (10/100);
        println!("Your name is: {}\n
            Your date of birth is: {}/{}/{}\n
            Your email address is {} \n
            Your phone number is: {}\n
            You have {} sibling(s) \n 
            You have {} child(ren) \n 
            You have been diagnosed with Diabetes({})\n 
            You are from {}\n 
            Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if (disease.to_lowercase() == "art") && (age > 58) && (number_of_children > 5) && (number_of_siblings > 5) && (village == "Akpabom") {
        let bill:i32 = 450_000 * (10/100);
        println!("Your name is: {}\n
            Your date of birth is: {}/{}/{}\n
            Your email address is {} \n
            Your phone number is: {}\n
            You have {} sibling(s) \n 
            You have {} child(ren) \n 
            You have been diagnosed with Arthritis({})\n 
            You are from {}\n 
            Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if disease.to_lowercase() == "al" {
        let bill:i32 = 1_200_000;
        println!("Your name is: {}\n
            Your date of birth is: {}/{}/{}\n
            Your email address is {} \n
            Your phone number is: {}\n
            You have {} sibling(s) \n 
            You have {} child(ren) \n 
            You have been diagnosed with Alzheimer({})\n 
            You are from {}\n 
            Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if disease.to_lowercase() == "ar" {
    let bill:i32 = 550_000;
    println!("Your name is: {}\n
        Your date of birth is: {}/{}/{}\n
        Your email address is {} \n
        Your phone number is: {}\n
        You have {} sibling(s) \n 
        You have {} child(ren) \n 
        You have been diagnosed with Arrhythmia({})\n 
        You are from {}\n 
        Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if disease.to_lowercase() == "ckd" {
    let bill:i32 = 1_500_000;
    println!("Your name is: {}\n
        Your date of birth is: {}/{}/{}\n
        Your email address is {} \n
        Your phone number is: {}\n
        You have {} sibling(s) \n 
        You have {} child(ren) \n 
        You have been diagnosed with Chroic Kidney Disease({})\n 
        You are from {}\n 
        Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if disease.to_lowercase() == "d" {
    let bill:i32 = 1_200_000;
    println!("Your name is: {}\n
        Your date of birth is: {}/{}/{}\n
        Your email address is {} \n
        Your phone number is: {}\n
        You have {} sibling(s) \n 
        You have {} child(ren) \n 
        You have been diagnosed with Diabetes({})\n 
        You are from {}\n 
        Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    }
    else if disease.to_lowercase() == "art" {
    let bill:i32 = 450_000;
    println!("Your name is: {}\n
        Your date of birth is: {}/{}/{}\n
        Your email address is {} \n
        Your phone number is: {}\n
        You have {} sibling(s) \n 
        You have {} child(ren) \n 
        You have been diagnosed with Arthritis({})\n 
        You are from {}\n 
        Your total bill is N{}\n ", name,day_of_birth,month_of_birth,year_of_birth,email,phone_number,number_of_siblings,number_of_children,disease.to_uppercase(),village,bill);
    } else {
        println!("We do not diagnose that treat that disease in this hospital!");
    }
}
