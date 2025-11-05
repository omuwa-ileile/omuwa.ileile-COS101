use std::io;

fn main() {
    println!("Calculate the roots of a quadratic equation");

    println!("Enter your value for a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a: f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your value for b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your value for c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f64 = input3.trim().parse().expect("Not a valid number");

    let d = b * b - (4.0 * a * c);
    
    if d > 0.0{
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("There are two distinct real roots which are: {:.2} and {:.2}", root1, root2);
    }
    else if d == 0.0{
        let root = -b / (2.0 * a);
        println!("There is one real root which is {:.2}", root);
    }
    else if d < 0.0 {
        println!("There are no real roots.");
    }
    
}
