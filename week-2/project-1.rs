fn main() {
    let p:f64 = 520_000_000.00;
    let r:f64 = 10.0;
    let n:f64 = 5.0;
    let base:f64 = 1.0 + (r / 100.0);
    let exponent:f64 = base.powf(n);

    // simple interest 
    let a = p * exponent;
    let amount = a.round();
    println!("Amount is ₦{}", amount);
    let ci = amount - p;
    println!("Compound Interest is ₦{}", ci);
}