fn main() {
    let sample_distance:f32 = 80.0;
    let sample_time:f32 = 2.0;
    let sample_speed = sample_distance / sample_time;

    println!("The car initally went {} kilometers every hour", sample_speed);

    let final_distance:f32 = 120.0;
    let final_distance_km = final_distance * 1.62;
    let final_time:f32 = 4.0;
    let final_speed = final_distance_km / final_time;

    println!("The car goes {} kilometers every hour if the distance is: \n{} kilometers and \nTime is {} hrs", final_speed, final_distance_km, final_time);
}
