use std::io;

fn main() {
    println!("Determine your employees annual incentive");

    println!("Enter the age of employee:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Is the employee experienced? (yes/no)");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience = input2.trim().to_lowercase();   

    if age >= 40.0 && experience == "yes"{
        println!("Your annual incentive is 1_560_000");
    }

    else if age < 40.0 && age >= 30.0 && experience == "yes"{
        println!("Your annual incentive is 1_480_000");
    }
 
    else if age < 28.0 && experience == "yes"{
        println!("Your annual incentive is 1_300_000");
    }

    else if experience == "no"{
        println!("Your annual incentive is 100_000");
    }
    else {
        println!("Invalid input. Please enter a valid age and experience status.")
    }
}
