fn main() {
    let initial_value : f64 = 210000.00;
    let depreciation_rate : f64 = 0.05;
    let years : f64 = 3.00;

    let depreciated_value = initial_value * (1.0 - depreciation_rate).powf(years as f64);
    println!("The value of the TV after {} years is ${:.2}", years, depreciated_value);
}
