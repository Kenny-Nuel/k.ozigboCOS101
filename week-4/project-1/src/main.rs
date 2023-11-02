fn main() {
    let sample_distance:i32 = 80;
    let sample_time:i32 = 2;
    let sample_speed = sample_distance / sample_time;

    println!("The car initally went {} miles every hour", sample_speed);

    let final_distance:i32 = 120;
    let final_time:i32 = 4;
    let final_speed = final_distance / final_time;

    println!("The car goes {} miles every hour if the distance is: \n{} miles and \nTime is {} hrs", final_speed, final_distance, final_time);
}
