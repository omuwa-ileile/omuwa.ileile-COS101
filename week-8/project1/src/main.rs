use std::io;

// Function to get user input as string
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Function to find APS level
fn find_aps_level(category: &str, job_title: &str) -> Option<&'static str> {
    // APS table stored as an array of tuples
    let table: [(&str, Vec<(&str, &str)>); 6] = [
        ("APS 1-2", vec![
            ("Office Administrator", "Intern"),
            ("Academic", "-"),
            ("Lawyer", "Paralegal"),
            ("Teacher", "Placement"),
        ]),
        ("APS 3-5", vec![
            ("Office Administrator", "Administrator"),
            ("Academic", "Research Assistant"),
            ("Lawyer", "Junior Associate"),
            ("Teacher", "Classroom Teacher"),
        ]),
        ("APS 5-8", vec![
            ("Office Administrator", "Senior Administrator"),
            ("Academic", "PhD Candidate"),
            ("Lawyer", "Associate"),
            ("Teacher", "Snr Teacher"),
        ]),
        ("EL1 8-10", vec![
            ("Office Administrator", "Office Manager"),
            ("Academic", "Post-Doc Researcher"),
            ("Lawyer", "Senior Associate 1-2"),
            ("Teacher", "Leading Teacher"),
        ]),
        ("EL2 10-13", vec![
            ("Office Administrator", "Director"),
            ("Academic", "Senior Lecturer"),
            ("Lawyer", "Senior Associate 3-4"),
            ("Teacher", "Deputy Principal"),
        ]),
        ("SES", vec![
            ("Office Administrator", "CEO"),
            ("Academic", "Dean"),
            ("Lawyer", "Partner"),
            ("Teacher", "Principal"),
        ]),
    ];

    // Loop through the APS table to find a match
    for (aps_level, roles) in table.iter() {
        for (cat, title) in roles.iter() {
            if cat.eq_ignore_ascii_case(category) && title.eq_ignore_ascii_case(job_title) {
                return Some(*aps_level);
            }
        }
    }

    None
}

fn main() {
    println!("=== APS Level Checker ===");

    // Collect user input
    let category = get_input("Enter job category (Office Administrator, Academic, Lawyer, Teacher):");
    let job_title = get_input("Enter job title:");

    // Find APS level
    match find_aps_level(&category, &job_title) {
        Some(level) => println!("\nAPS Level: {}", level),
        None => println!("\nJob title not found in the APS table."),
    }
}
