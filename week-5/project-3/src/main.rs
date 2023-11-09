use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    let price_of_p:f32 = 3_200.0;
    let price_of_f:f32 = 3_000.0;
    let price_of_a:f32 = 2_500.0;
    let price_of_e:f32 = 2_000.0;
    let price_of_w:f32 = 2_500.0;

    let menu = "    Poundo Yam / Edinkaiko Soup : ₦3,200 (P)\n
    Fried Rice & Chicken: ₦3,000 (F) \n
    Amala & Ewedu Soup: ₦2,500 (A) \n
    Eba & Egusi Soup: ₦2,000 (E) \n
    White Rice & Stew: ₦2,500 (W)";

    println!("{}", menu);

    println!("So, what would you like ? ");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let food = input1.trim();

    println!("How many portions ? ");
    io::stdin().read_line(&mut input2).expect("Invalid string");
    let quantity:f32 = input2.trim().parse().expect("Invalid integer");

    if food.to_lowercase() == "p" {
        let bill:f32 = price_of_p * quantity;
        if bill > 10_000.0 {
            let discount:f32 = 0.05 * bill;
            let total_bill:f32 = bill - discount;
            println!("Your bill is ₦{}", total_bill);
        }
        else {
            println!("No discount for you!")
        }
    } else if food.to_lowercase() == "f" {
        let bill:f32 = price_of_f * quantity;
        if bill > 10_000.0 {
            let discount:f32 = 0.05 * bill;
            let total_bill:f32 = bill - discount;
            println!("Your bill is ₦{}", total_bill);
        }
        else {
            println!("No discount for you!")
        }
    } else if food.to_lowercase() == "a" {
        let bill:f32 = price_of_a * quantity;
        if bill > 10_000.0 {
            let discount:f32 = 0.05 * bill;
            let total_bill:f32 = bill - discount;
            println!("Your bill is ₦{}", total_bill);
        }
        else {
            println!("No discount for you!")
        }
    } else if food.to_lowercase() == "e" {
        let bill:f32 = price_of_e * quantity;
        if bill > 10_000.0 {
            let discount:f32 = 0.05 * bill;
            let total_bill:f32 = bill - discount;
            println!("Your bill is ₦{}", total_bill);
        }
        else {
            println!("No discount for you!")
        }
    } else if food.to_lowercase() == "w" {
        let bill:f32 = price_of_w * quantity;
        if bill > 10_000.0 {
            let discount:f32 = 0.05 * bill;
            let total_bill:f32 = bill - discount;
            println!("Your bill is ₦{}", total_bill);
        }
        else {
            println!("No discount for you!")
        }
    }
}
