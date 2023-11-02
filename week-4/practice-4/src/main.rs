fn main() {
    let fullname = "Kenechukwu Nuel Ozigbo";
    let department = "Software Engineering";
    let uni = "Pan-Atlantic Univerisity";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is: {}", fullname);
    // check length
    println!("The length of my full name is: {}", fullname.len());
    println!("I am student of {} department", department);
    println!("{}", school);
    println!("{}", uni);
}
