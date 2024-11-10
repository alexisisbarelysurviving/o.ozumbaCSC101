use std::io;

fn main()

{
    let mut input = String::new(); 

    println!("\nEnter Your Height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid number");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >=150.0 && height <= 170.0
    {
        println!("Average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You're tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("Dwarf lmaoo");
    }
    else 
    {
        println!("abnormal height");
    }
}
