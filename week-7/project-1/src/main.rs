use std::io;

fn main() {
    println!("**** Rust program which can calculate the area of certain shapes ***");
    let options = "Trapezium (T) \n 
    Rhombus (R)\n
    Parallelogram (P) \n
    Cube (C)";
    println!("This program can calculate the areas of {}", options);
    println!("You can also get the volume of a cylinder (CY)");
    println!();
    println!();
    println!();

    let mut input1 = String::new();
    println!("So, what's it gonna be? ");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let choice = input1.trim();

    if choice.to_lowercase() == "t" {
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();

        println!("What is the height of the trapezium? ");
        io::stdin().read_line(&mut input2).expect("Invalid string");
        let height:f64 = input2.trim().parse().expect("Invalid integer");

        println!("Base 1 length: ");
        io::stdin().read_line(&mut input3).expect("Invalid string");
        let first_base = input3.trim().parse().expect("Invalid integer");

        println!("Base 2 length: ");
        io::stdin().read_line(&mut input4).expect("Invalid string");
        let second_base = input4.trim().parse().expect("Invalid integer");
        area_of_trapezium(height,first_base,second_base);
    } 

    else if choice.to_lowercase() == "r" {
        let mut input5 = String::new();
        let mut input6 = String::new();

        println!("Diagonal 1 length: ");
        io::stdin().read_line(&mut input5).expect("Invalid string");
        let first_diagonal = input5.trim().parse().expect("Invalid integer");

        println!("Diagonal 2 length: ");
        io::stdin().read_line(&mut input6).expect("Invalid string");
        let second_diagonal = input6.trim().parse().expect("Invalid integer");
        area_of_rhombus(first_diagonal,second_diagonal);
    } 

    else if choice.to_lowercase() == "p" {
        let mut input7 = String::new();
        let mut input8 = String::new();

        println!("Base length: ");
        io::stdin().read_line(&mut input7).expect("Invalid string");
        let base_length = input7.trim().parse().expect("Invalid integer");

        println!("Altitude: ");
        io::stdin().read_line(&mut input8).expect("Invalid string");
        let altitude = input8.trim().parse().expect("Invalid integer");
        area_of_parallelogram(base_length,altitude);
    } 

    else if choice.to_lowercase() == "c" {
        let mut input9 = String::new();

        println!("Length of the sides: ");
        io::stdin().read_line(&mut input9).expect("Invalid string");
        let length_of_sides:f64 = input9.trim().parse().expect("Invalid integer");
        area_of_cube(length_of_sides)
    } 

    else if choice.to_lowercase() == "cy" {
        let mut input10 = String::new();
        let mut input11 = String::new();

        println!("Radius of the circle: ");
        io::stdin().read_line(&mut input10).expect("Invalid string");
        let radius = input10.trim().parse().expect("Invalid integer");

        println!("Height: ");
        io::stdin().read_line(&mut input11).expect("Invalid string");
        let height = input11.trim().parse().expect("Invalid integer");
        volume_of_cylinder(radius,height);
    }

    else {
        println!("Sorry, I can't carry out that operation.");
    }
}

fn area_of_trapezium(height:f64, first_base:f64, second_base:f64) {
    let mut input12 = String::new();
    println!("What unit are your values in? ");
    io::stdin().read_line(&mut input12).expect("Invalid string");
    let unit = input12.trim();

    let area_of_trapezium:f64 = (height * first_base * second_base ) / 2.0;
    println!("The area of the trapezium is: {} {}^2", area_of_trapezium, unit);
}

fn area_of_rhombus(first_diagonal:f64, second_diagonal:f64) {
    let mut input12 = String::new();
    println!("What unit are your values in? ");
    io::stdin().read_line(&mut input12).expect("Invalid string");
    let unit = input12.trim();

    let area_of_rhombus:f64 = (first_diagonal * second_diagonal ) / 2.0;
    println!("The area of the rhombus is: {} {}^2", area_of_rhombus, unit);
}

fn area_of_parallelogram(base_length:f64, altitude:f64) {
    let mut input12 = String::new();
    println!("What unit are your values in? ");
    io::stdin().read_line(&mut input12).expect("Invalid string");
    let unit = input12.trim();

    let area_of_parallelogram = base_length * altitude;
    println!("The area of the parallelogram is: {} {}^2", area_of_parallelogram, unit);
}

fn area_of_cube(length_of_sides:f64) {
    let mut input12 = String::new();
    println!("What unit are your values in? ");
    io::stdin().read_line(&mut input12).expect("Invalid string");
    let unit = input12.trim();

    let area_of_cube:f64 = length_of_sides.powf(2.0) * 6.0;
    println!("The area of the cube is: {} {}^2", area_of_cube, unit);
}

fn volume_of_cylinder(radius:f64, height:f64) {
    let mut input12 = String::new();
    println!("What unit are your values in? ");
    io::stdin().read_line(&mut input12).expect("Invalid string");
    let unit = input12.trim();

    let volume_of_cylinder:f64 = ((radius.powf(2.0)) * height ) * 3.142;
    println!("The volume of cylinder is: {} {}^3", volume_of_cylinder, unit);
}
// write functions for the differents areas and volume
