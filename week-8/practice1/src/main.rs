fn main() {
    //Using vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    //Using macro
    let v = vec!["Ozibaby","Chinyere","Kelechi","Chidera","Mine"];

    //printing the size of the vector
    println!("\nThe length of vec macro is: { }",v.len());
}
