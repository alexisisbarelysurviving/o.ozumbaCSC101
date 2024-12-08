fn main() {
    let v: Vec<i64> = Vec::new();
    println!("\nThe lenght of the Vec::new is: {}",v.len());

    let v = vec! ["grace", "effiong", "basil", "karee", "sultana"];
    println!("\nThe lenght of vec macro is: {}",v.len()); 
}
