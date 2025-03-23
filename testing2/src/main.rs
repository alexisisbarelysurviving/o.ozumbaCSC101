fn main() {
    let countries = ["USA", "Canada", "Mexico"];
    let cities = ["New York", "Toronto", "Mexico City"];

    for i in 0..countries.len() {
        println!("{}: {}", countries[i], cities[i]); // One placeholder for each value
    }
}
