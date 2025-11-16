use std::io;

fn trapezium(h:f64, b1:f64, b2:f64) -> f64 {
    (h / 2.0) * (b1 + b2)
}

fn rhombus(d1:f64, d2:f64) -> f64 {
    0.5 * d1 * d2
}

fn parallelogram(b:f64,a:f64) -> f64 {
    b * a
}

fn cube(l:f64) -> f64 {
    6.0 * (l * l)
}

fn cylinder(r:f64,h:f64) -> f64 {
    (22.0 / 7.0)* (r * r) * h
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Not a valid number")
}

fn main() {
    let options = [
   "Area of Trapezium",
   "Area of Rhombus",
   "Area of Parallelogram",
   "Area of Cube",
   "Volume of Cylinder",
];
    println!("Hello user!");
    println!("Enter the number code of what you would like to calculate");
    for (i, item) in options.iter().enumerate(){
    println!("{}: {}",i + 1, item);
    }

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:u32 = input1.trim().parse().expect("Not a valid number");
    println!("You selected {}",input1);
   

   match input1{
    1 => {
        let h = read_input("Enter height:");
        let b1 = read_input("Enter base1:");
        let b2 = read_input("Enter base2:");
        println!("Area of Trapezium = {}", trapezium(h, b1, b2));
    }

    2 => {
        let d1 = read_input("Enter diagonal1:");
        let d2 = read_input("Enter diagonal2:");
        println!("Area of Rhombus = {}", rhombus(d1, d2));
    }

    3 => {
        let b = read_input("Enter base:");
        let a = read_input("Enter altitude:");
        println!("Area of Parallelogram = {}", parallelogram(b, a));
    }

    4 => {
        let l = read_input("Enter length of side");
        println!("Area of Cube = {}", cube(l));
    }

    5 => {
        let r = read_input("Enter radius:");
        let h = read_input("Enter height:");
        println!("Volume of Cylinder = {}",cylinder(r, h));
    }

    _ => println!("Invalid choice!"),
   }
}
