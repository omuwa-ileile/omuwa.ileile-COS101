use std::io::{self, Read};
use std::fs::File;

fn read_sql_file(filename: &str) {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    println!("{}", contents);
}

fn main() {
    let mut role = String::new();

    println!("Enter user role (admin, project_manager, employee, customer, vendor):");
    io::stdin().read_line(&mut role).expect("Failed to read input");

    let role = role.trim().to_lowercase();

    if role == "admin" {
        println!("DATABASE STRUCTURE:");
        read_sql_file("globacom_dbase.sql");

    } else if role == "project_manager" {
        println!("PROJECT TABLE:");
        read_sql_file("project_table.sql");

    } else if role == "employee" {
        println!("STAFF TABLE:");
        read_sql_file("staff_table.sql");

    } else if role == "customer" {
        println!("CUSTOMER TABLE:");
        read_sql_file("customer_table.sql");

    } else if role == "vendor" {
        println!("DATA PLAN TABLE:");
        read_sql_file("dataplan_table.sql");

    } else {
        println!("Invalid role entered");
    }
}

