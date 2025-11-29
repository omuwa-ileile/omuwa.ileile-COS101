use std::fs::File;
use std::io::Write;

fn main() {
    let name_of_commissioner = [
        "Aigbogun Alamba Daudu",
        "Murtala Afees Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministry = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zone = [
        "South West",
        "North West",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("Convicted_minister.txt")
        .expect("Failed to create file");

    // Write the table header
    writeln!(
        file,
        "{:<5} {:<30} {:<20} {:<20}",
        "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE"
    )
    .expect("Failed to write header");

    // Write each row
    for i in 0..name_of_commissioner.len() {
        writeln!(
            file,
            "{:<5} {:<30} {:<20} {:<20}",
            i + 1,
            name_of_commissioner[i],
            ministry[i],
            geopolitical_zone[i]
        )
        .expect("Failed to write row");
    }

    println!("File created successfully!");
}