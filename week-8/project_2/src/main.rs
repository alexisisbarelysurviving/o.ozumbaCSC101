
use std::io::Write;

fn main() {
    // Data
    let name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edmoh"];
    let matric_number = vec!["ACC10211111", "ECO1011101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer Science", "Electrical Engineering", "Mechanical Engineering"];
    let level = vec![300, 100, 200, 200, 100];

    // Print header
    println!("\nStudent Information:");
    println!("{:<20} {:<15} {:<25} {:<5}", "Name", "Matric Number", "Department", "Level");

    // Print student details
    for i in 0..name.len() {
        println!("{:<20} {:<15} {:<25} {:<5}", name[i], matric_number[i], department[i], level[i]);
    }

    // Write to file
    let mut file = std::fs::File::create("data.txt").expect("File creation failed");

    // File header
    let mut content = String::from("PAU SMIS\n");
    content.push_str("Name, Matric Number, Department, Level\n");

    // Add student data
    for i in 0..name.len() {
        content.push_str(&format!("{}, {}, {}, {}\n", name[i], matric_number[i], department[i], level[i]));
    }

    // Write all data at once
    file.write_all(content.as_bytes()).expect("Write failed");

    println!("\nData written to file.");
}
