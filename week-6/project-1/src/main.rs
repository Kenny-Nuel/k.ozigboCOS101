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

    let mut eligible_students:i32 = 0;

    while eligible_students < 150 {
        println!("*****This program can only be executed for 150 eligible candidates*****");
        println!("Current number of researchers this program has been executed for: {}", eligible_students);
        eligible_students += 1;

        println!("Please, input your name as it appears on your student ID card: ");
        io::stdin().read_line(&mut input1).expect("Invalid string");
        let name = input1.trim();

        println!("Your school email address: ");
        io::stdin().read_line(&mut input2).expect("Invalid string");
        let email = input2.trim();

        println!("Your department: ");
        io::stdin().read_line(&mut input3).expect("Invalid string");
        let department = input3.trim();

        println!("Your state of origin: ");
        io::stdin().read_line(&mut input4).expect("Invalid string");
        let state_of_origin = input4.trim();

        println!("Are you currently a course rep? (Yes or No) ");
        io::stdin().read_line(&mut input5).expect("Invalid string");
        let current_position = input5.trim();

        println!("Your current class(level)");
        io::stdin().read_line(&mut input6).expect("Invalid string");
        let current_class = input6.trim();

        println!("Your current CGPA: ");
        io::stdin().read_line(&mut input7).expect("Invalid string");
        let current_cgpa:f64 = input7.trim().parse().expect("Invalid float");

        if current_class.to_lowercase() == "200 level" || current_class.to_lowercase() == "300 level" || current_class.to_lowercase() == "400 level" && current_cgpa > 4.0 && current_position.to_lowercase() != "yes" {
            println!("You can vote 👏");
            println!("      Your name is {}\n 
                    Your email is {}\n 
                    Your are studying {}\n
                    Your are from {} state",name,email, department, state_of_origin);
        } else {
            println!("Sorry, you are not eligble to vote");
        }

        else if current_class == "200" || current_class.to_lowercase() == "300" || current_class.to_lowercase() == "400" && current_cgpa > 4.0 && current_position.to_lowercase() != "yes" {
            println!("You can vote 👏");
            println!("      Your name is {}\n 
                    Your email is {}\n 
                    Your are studying {}\n
                    Your are from {} state",name,email, department, state_of_origin);
        } else {
            println!("Sorry, you are not eligble to vote");
        }


        println!("Any other student? (Yes or No)");
        io::stdin().read_line(&mut input8).expect("Invalid string");
        let continue_loop = input8.trim();

        if continue_loop.to_lowercase() == "yes" {
            println!("Alright. Next student!");
            continue;
        } else {
            println!("Thank you for using my program.");
            break;
        }
    }
    
}
