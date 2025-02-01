use std::io;
use std::io::Read;

fn main(){
let mut input1 = String::new();

println!("Enter Position");
io::stdin().read_line(&mut input1).expect("Failed to read input");
let position = input1.trim();

if position == "Administrator"{
    let mut file = std::fs::File::open("globalcom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}else if position == "Project Manager"{
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}else if position == "Employee"{
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}else if position == "Customer"{
    let mut file = std::fs::File::open("customertable_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}else if position == "Vendor"{
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
}
