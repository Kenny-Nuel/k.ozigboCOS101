fn main() {
    let p:f64 = 1000.0;
    let r:f64 = 1.0;
    let t:f64 = 2.0;
    let base:f64 = 1.0 + (r / 100.0);
    let exponent:f64 = base.powf(t);

    // simple interest 
    let a = p * exponent;
    let amount = a.round();
    println!("Amount is ₦{}", amount);
    let si = a - p;
    println!("Simple Interest is ₦{}", si);
}