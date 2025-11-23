use std::io;

fn main() {
    let mut candidates: Vec<(String, u32)> = Vec::new();
    let mut input = String::new();

    println!("Enter the number of candidates:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let count: usize = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    for i in 1..=count {
        println!("\nEnter name for candidate {}:", i);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let name = input.trim().to_string();
        input.clear();

        println!("Enter years of programming experience for {}:", name);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let experience: u32 = input.trim().parse().expect("Please enter a valid number");
        input.clear();

        candidates.push((name, experience));
    }

    // Assume the first candidate is highest initially
    let mut highest = &candidates[0];

    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = candidate;
        }
    }

    println!("\n-----------------------------------");
    println!(
        "The candidate with the highest experience is {} with {} years.",
        highest.0, highest.1
    );
}