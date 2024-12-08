use std::io;

fn main() {
 let mut experience : Vec<i32> = Vec::new();
 let mut names : Vec<String> = Vec::new();
 println!("The experience vector has element {}",experience.len());

 let mut number_of_people_for_interview_a = String::new();
 println!("How many people are here for the interview");
 io::stdin().read_line(&mut number_of_people_for_interview_a).expect("Failed to input");
 let number_of_people_for_interview:i32 = number_of_people_for_interview_a.trim().parse().expect("invalid input");
 
 for count in 0..number_of_people_for_interview{
    
    let mut input2 = String::new();
    println!("Enter name for user {}", count+1);
    io::stdin().read_line(&mut input2).expect("Failed to input");
    let new_names:String = input2.trim().parse().expect("invalid input");
    names.push(new_names);

    let mut input1 = String::new();
    println!("Enter years of experience for user {}", count+1);
    io::stdin().read_line(&mut input1).expect("Failed to input");
    let new_expirience:i32 = input1.trim().parse().expect("invalid input");
    experience.push(new_expirience);

    
  }
  let mut count=1;
  let mut maxValue:i32 = *experience.iter().max().unwrap_or(&0);
  //println!("{}", maxValue);
  
  for i in 0..experience.len()
  {
    if maxValue == experience[i] {
        println!("The person with the highest experience is ");
        println!("{} with {} years of experience", names[i],experience[i]);
    }
    count+=1;
  }
  
}