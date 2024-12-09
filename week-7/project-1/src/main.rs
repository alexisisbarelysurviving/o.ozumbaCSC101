use std::io;
fn main() {

    println!("APS LEVEL CHECKER");

    println!("Enter profession");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession:String = profession.trim().parse().expect("Failed to input"); 


    println!("Enter years of expirience");
    let mut years_of_expirience = String::new();
    io::stdin().read_line(&mut years_of_expirience).expect("Failed to read input");
    let years_of_expirience:i32 = years_of_expirience.trim().parse().expect("Failed to input"); 


    let years = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    if profession == "office_administrator" && (years_of_expirience == 1 || years_of_expirience == 2){
        println!("User is an Intern");
    }else if profession == "office_administrator" && (years_of_expirience == 3 || years_of_expirience == 4 || years_of_expirience == 5){
        println!("User is an Administrator");
    }else if profession == "office_administrator" && (years_of_expirience == 5|| years_of_expirience == 6 || years_of_expirience == 7 || years_of_expirience == 8){
        println!("User is a Senior Administrator");
    }else if profession == "office_administrator" && (years_of_expirience == 8|| years_of_expirience == 9 || years_of_expirience == 10){
        println!("User is an Office manager");
    }else if profession == "office_administrator" && (years_of_expirience == 10|| years_of_expirience == 11 || years_of_expirience == 12 || years_of_expirience == 13){
        println!("User is a Director");
    }else if profession == "office_administrator" && years_of_expirience > 13{
        println!("User is a CEO");
    }else if profession == "academic" && (years_of_expirience == 1|| years_of_expirience == 2){
        println!("User does not qualify for a title");
    }else if profession == "academic" && (years_of_expirience == 3 || years_of_expirience == 4 || years_of_expirience == 5){
        println!("User is a Research assistant");
    }else if profession == "academic" && (years_of_expirience == 5|| years_of_expirience == 6 || years_of_expirience == 7 || years_of_expirience == 8){
        println!("User is a PhD Candidate");
    }else if profession == "academic" && (years_of_expirience == 8|| years_of_expirience == 9 || years_of_expirience == 10){
        println!("User is Post Doc Reasearcher");
    }else if profession == "academic" && (years_of_expirience == 10|| years_of_expirience == 11 || years_of_expirience == 12 || years_of_expirience == 13){
        println!("User is a Senior Lecturer"); 
    }else if profession == "academic" && years_of_expirience > 13{
        println!("User is a Dean");
    }else if profession == "lawyer" && (years_of_expirience == 1|| years_of_expirience == 2){
        println!("User is a Paralegal");
    }else if profession == "lawyer" && (years_of_expirience == 3 || years_of_expirience == 4 || years_of_expirience == 5){
        println!("User is a Junior Associate");
    }else if profession == "lawyer" && (years_of_expirience == 5|| years_of_expirience == 6 || years_of_expirience == 7 || years_of_expirience == 8){
        println!("User is an Associate");
    }else if profession == "lawyer" && (years_of_expirience == 8|| years_of_expirience == 9 || years_of_expirience == 10){
        println!("User is a Senior Associate 1-2");
    }else if profession == "lawyer" && (years_of_expirience == 10|| years_of_expirience == 11 || years_of_expirience == 12 || years_of_expirience == 13){
        println!("User is a Senior Associate 3-4");
    }else if profession == "lawyer" && years_of_expirience > 13{
        println!("User is a partner");
    }else if profession == "teacher" && (years_of_expirience == 1|| years_of_expirience == 2){
        println!("User is a Placement");
    }else if profession == "teacher" && (years_of_expirience == 3 || years_of_expirience == 4 || years_of_expirience == 5){
        println!("User is a Classroom Teacher");
    }else if profession == "teacher" && (years_of_expirience == 5|| years_of_expirience == 6 || years_of_expirience == 7 || years_of_expirience == 8){
        println!("User is a Senior Teacher");
    }else if profession == "teacher" && (years_of_expirience == 8|| years_of_expirience == 9 || years_of_expirience == 10){
        println!("User is a Leading Teacher");
    }else if profession == "teacher" && (years_of_expirience == 10|| years_of_expirience == 11 || years_of_expirience == 12 || years_of_expirience == 13){
        println!("User is a Deputy principal");
    }else if profession == "teacher" && years_of_expirience > 13{
        println!("User is a Principa");
    }
}
    