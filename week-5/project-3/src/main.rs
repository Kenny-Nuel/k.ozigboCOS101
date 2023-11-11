use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    let price_of_p:i32 = 3_200;
    let price_of_f:i32 = 3_000;
    let price_of_a:i32 = 2_500;
    let price_of_e:i32 = 2_000;
    let price_of_w:i32 = 2_500;

    let mut no_bill:i32 = 0;

    let menu = "    Poundo Yam / Edinkaiko Soup : ₦3,200 (P)\n
    Fried Rice & Chicken: ₦3,000 (F) \n
    Amala & Ewedu Soup: ₦2,500 (A) \n
    Eba & Egusi Soup: ₦2,000 (E) \n
    White Rice & Stew: ₦2,500 (W)";

    println!("{}", menu);

    loop {
        println!("So, what would you like ? ");
        io::stdin().read_line(&mut input1).expect("Invalid string");
        let food = input1.trim();

        println!("How many portions ? ");
        io::stdin().read_line(&mut input2).expect("Invalid string");
        let quantity:i32 = input2.trim().parse().expect("Invalid integer");

        println!("Anything else ? (Yes or No)");
        io::stdin().read_line(&mut input3).expect("Invalid string");
        let reply = input3.trim();

        if food.to_lowercase() == "p" {
            let bill:i32 = price_of_p * quantity;
            no_bill += bill; 
        } else if food.to_lowercase() == "f" {
            let bill:i32 = price_of_f * quantity;
            no_bill += bill;
        } else if food.to_lowercase() == "a" {
            let bill:i32 = price_of_a * quantity;
            no_bill += bill;
        } else if food.to_lowercase() == "e" {
            let bill:i32 = price_of_e * quantity;
            no_bill += bill;
        } else if food.to_lowercase() == "w" {
            let bill:i32 = price_of_w * quantity;
            no_bill += bill;
        }

        if reply.to_lowercase() == "yes" {
            continue;
        } else if reply.to_lowercase() == "no" {
            break;
        }
    }

    if no_bill >= 10_000 {
        let discount:i32 =  no_bill * (5/100);
        let total_bill:i32 = no_bill - discount;
        println!("Your total bill is ₦{}", total_bill)
    } else {
        println!("Your total bill is ₦{}", no_bill);
    }
}
