fn main() {
    let p:f64 = 520000000.0;
    let r:f64 = 10.0;
    let t:f64 = 5.0;

    //simple intrest 
    let a = p * ( 10.0 + (r / 100.0)) * t;
    println!("Amount is {}", a);
    let si = a - p;
    println!("Simple intrest is {}", si );

}
