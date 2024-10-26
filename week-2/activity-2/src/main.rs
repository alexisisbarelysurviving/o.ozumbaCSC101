fn main() {
    let toshiba_amount: f64 = 450000.00;
    let mac_amount: f64 = 1500000.00;
    let hp_amount: f64 = 750000.00;
    let dell_amount: f64 = 2850000.00;
    let acer_amount: f64 = 250000.00;
    let sum: f64 =  toshiba_amount + mac_amount + hp_amount + dell_amount + acer_amount;

    let toshiba_qty: f64 = 2.00;
    let mac_qty: f64 = 1.00;
    let hp_qty: f64 = 3.00;
    let dell_qty: f64 = 3.00;
    let acer_qty: f64 = 1.00;
    let average: f64 = sum/(toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty);
    println!(" Sum is {} and Average is {} ", sum, average);
}
