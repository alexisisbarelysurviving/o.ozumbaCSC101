use std::io::Write;

fn main () {
    let name = vec!["Oluchi Mordi ", "Adams Aliyu  ", "Shania Bolade", "Adekunle Gold", "Blanca Edmoh "];
    let matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting            ", "Economics             ", "Computer science      ", "Electrical engineering", "Mechanical engineering"];
    let level = vec![300, 100, 200, 200, 100];

    let mut file = std::fs::File::create("pausims.txt").expect("create failed");
    file.write_all("\t\t\tPAU SMIS\n".as_bytes()).expect("write failed");
    
    print!("\nWriting to file...\n");
    let header = "     NAME    |  MAT NO.  |      DEPARTMENT      | LEVEL\n";
    file.write_all(header.as_bytes()).expect("write failed");
    let headerline = "________________________________________________________\n";
    file.write_all(headerline.as_bytes()).expect("write failed");
    
    for i in 0..level.len()
    {
        let data = name[i].to_owned() + "|" + matric_number[i] + "|" + department[i] + "|" + &level[i].to_string() + "\n";
        file.write_all(data.as_bytes()).expect("write failed");
    
    }
    
    println!("\nData written to file.");
}





