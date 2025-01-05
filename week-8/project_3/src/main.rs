use std::fs::File;
use std::io::Write;

fn main() {
    // Dataset 1: Names of Commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical Zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Displaying the table header
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "Commissioner", "Ministry", "Zone");

    // Displaying and saving data row by row
    let mut file = File::create("efcc_report.txt").expect("File creation failed");
    writeln!(file, "{:<5} {:<30} {:<20} {:<15}", "S/N", "Commissioner", "Ministry", "Zone")
        .expect("Write to file failed");

    for i in 0..commissioners.len() {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );

        writeln!(
            file,
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        )
        .expect("Write to file failed");
    }

    println!("\nData successfully written to 'efcc_report.txt'");
}
