
 use std::io;
 struct Gadgets {
    laptop_brand:String,
    price:u32,
}
 fn main(){
    let gadget1 = Gadgets {
        laptop_brand:String::from("HP"),
        price:650000,
    };
    let gadget2 = Gadgets {
        laptop_brand:String::from("IBM"),
        price:755000,
    };
    let gadget3 = Gadgets {
        laptop_brand:String::from("TOSHIBA"),
        price:550000,
    };
    let gadget4 = Gadgets {
        laptop_brand:String::from("DELL"),
        price:850000,
    };
    display(&gadget1);
    display(&gadget2);
    display(&gadget3);
    display(&gadget4);
 }

 fn display (gadget: &Gadgets){
    println!("Availability: Brand = {}, Price = {}",gadget.laptop_brand, gadget.price);
 
 let mut input1 = String::new();
 println!("place_order");
 io::stdin().read_line(&mut input1).expect("Failed to read input");
 let place_order: String =  input1.trim().parse().expect("Failed to input");

 let mut total:u32 = &gadget1 * &gadget2 * &gadget3 * &gadget4; 
 println!("Bill: N{}", total); 
}


 