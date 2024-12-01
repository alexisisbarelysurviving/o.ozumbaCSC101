
use std::io;

fn main() {
    println!("This calculator performs the following calculations:");
    println!("1= Area of trapezium");
    println!("2= Area of rhombus");
    println!("3 = Area of parallelogram");
    println!("4= Area of cube formula");
    println!("5 = Volume of cylinder formula");

    let mut input1 = String::new();


    println!("Enter choice_of_calculation (1, 2, 3, 4, 5):");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1: i32 = input1.trim().parse().expect("Failed to input");

    if input1 == 1 {
        let mut height = String::new();
        let mut base_1 = String::new();
        let mut base_2 = String::new();
        let mut result:i32;

      println!("Enter height:");
      io::stdin().read_line(&mut height).expect("Failed to read input");
      let height: i32 = height.trim().parse().expect("Failed to input");

      println!("Enter base_1:");
      io::stdin().read_line(&mut base_1).expect("Failed to read input");
      let base_1: i32 = base_1.trim().parse().expect("Failed to input");

      println!("Enter base_2:");
      io::stdin().read_line(&mut base_2).expect("Failed to read input");
      let base_2: i32 = base_2.trim().parse().expect("Failed to input");

      let result = height/2*(base_1+base_2);
      println!("Area of trapezium is {}", result);
    }else if input1 == 2 {

        let mut diagonal_1 = String::new();
        let mut diagonal_2 = String::new();
        let mut result:i32;

      println!("Enter diagonal_1:");
      io::stdin().read_line(&mut diagonal_1).expect("Failed to read input");
      let diagonal_1: i32 = diagonal_1.trim().parse().expect("Failed to input");

      println!("Enter diagonal_2:");
      io::stdin().read_line(&mut diagonal_2).expect("Failed to read input");
      let diagonal_2: i32 = diagonal_2.trim().parse().expect("Failed to input");

      let result = (diagonal_1*diagonal_2)/2;
      println!("Area of rhombus is {}", result);
    }else if input1 == 3 {

        let mut base = String::new();
        let mut altitude = String::new();
        let mut result:i32;

      println!("Enter base:");
      io::stdin().read_line(&mut base).expect("Failed to read input");
      let base: i32 = base.trim().parse().expect("Failed to input");

      println!("Enter altitude");
      io::stdin().read_line(&mut altitude).expect("Failed to read input");
      let altitude: i32 = altitude.trim().parse().expect("Failed to input");

      let result = base*altitude;
      println!("Area of parallelogram is {}", result);
    }else if input1 == 4 {

        let mut length = String::new();
        let mut result:i32;

        println!("Enter lenght of side:");
        io::stdin().read_line(&mut length).expect("Failed to read input");
        let length: i32 = length.trim().parse().expect("Failed to input");

        let result = 6*(length^2);
        println!("Area of cube is {}", result);
    }else if input1 == 5 {
        let mut height = String::new();
        let mut radius = String::new();
        let mut result:i32;

      println!("Enter height:");
      io::stdin().read_line(&mut height).expect("Failed to read input");
      let height: i32 = height.trim().parse().expect("Failed to input");

      println!("Enter radius");
      io::stdin().read_line(&mut radius).expect("Failed to read input");
      let radius: i32 = radius.trim().parse().expect("Failed to input");

      let result = 22/7*radius^2*height;
      println!("Volume of cylinder is {}", result);
    }
}
