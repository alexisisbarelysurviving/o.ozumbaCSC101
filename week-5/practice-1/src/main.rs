fn main() {
    let name = "Alexis Ozumba";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "km 52 lekki-ekpe expressway, ibeju lekki, lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni,addr);


    let department:&'static str = "Computer science";
    let school:&'static str = "School of Science and technology";
    println!("department: {}, \nSchool: {}", department,school);
}
