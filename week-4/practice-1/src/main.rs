fn main() {
    let name = "Ozigbo Kenechukwu";
    let uni:&str = "Pan-Atlantic Univerisity";
    let addr:&str = "Km 53 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni, addr);

    let _department:&'static str = "Software Engineering";
    let _school:&'static str =  "School of Science and Technology";
}
