use std::io;
fn main() {
    
    println!("CARBON FOOTPRINT CALCULATOR");
    

    let milage_multiplier:f32 = 0.79;
    let gas_multiplier:f32 = 105.00;
    let plane_multiplier:f32 = 1100.00;
    let electricity_multiplier:f32 = 105.00;

    let mut total = 0.0;

    for x in 0..3 {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new();
        let mut input6 = String::new();

        println!("USER {}", x+1);
        println!("What is your yearly milage?");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let milage: f32 = input1.trim().parse().expect("Failed to input");

        println!("How much do you spend on gas (fuel) yearly?");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let gas: f32 = input2.trim().parse().expect("Failed to input");

        println!("How many times do you use a plane yearly?");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let plane: f32 = input3.trim().parse().expect("Failed to input");

        println!("How much electricity do you use yearly?");
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let electricity: f32 = input4.trim().parse().expect("Failed to input");

        let sub_total = milage * milage_multiplier + gas * gas_multiplier + plane * plane_multiplier + electricity * electricity_multiplier;
        total += sub_total;
    }

    println!("CARBON FOOTPRINT FOR GROUP 15 IS {} kg CO2",total);  
    let mut input7 = String::new();


    if total >= 22000.00{
        println!("WOULD YOU LIKE TO SEE WAYS TO CUT DOWN YOUR CARBON FOOTPRINT? [TRUE] [FALSE]");  
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let cut_down: bool = input7.trim().parse().expect("Failed to input");

        if cut_down {
            println!("1. Eat low on the food chain. (fruits)
                      2. Choose organic and local foods that are in season. 
                      3. Buy foodstuffs in bulk when possible using your own reusable container.
                      4. Reduce your food waste by planning meals ahead of time and freezing and consuming excess.
                      5. Compost your food waste if possible.
                      6. Wash your clothing in cold water.
                      source: news.climate.columbia.edu");
        }
    }
}

