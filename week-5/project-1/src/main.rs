use std::io;

fn main() {

    println!("Resturant Menu");
    println!("P = Pounded Yam/Edinkaiko soup - N3200");
    println!("F = Fried Rice & Chicken - N3000");
    println!("A = Amala & Ewedu soup - N2500");
    println!("E = Eba & Egusi soup - N2000");
    println!("W = White Rice & Stew - N2500");


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter food_choice (P, F, A, E, W):");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("Enter quantity_of_food");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quantity_of_food: i32 = input2.trim().parse().expect("Failed to input");

    let choice = input3.trim();
    let mut price:i32 = 0;

    if choice == "P"{
        price = 3200;
    }else if choice == "F"{
       price = 3000;
    }else if choice == "A"{
       price = 2500;
    }else if choice == "E"{
       price = 2000;
    }else if choice == "W"{
       price = 2500;
    }else {
       println!("No food combination selected");
    }

    let mut total:i32 = price * quantity_of_food;  

    if total > 10000 {
        let discount = total * 5/100;
        total-= discount;
        println!("You have been rewarded a 5% discount!", discount);
    }

    println!("Customers bill: N{}", total);
}
    



    