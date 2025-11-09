use std::io;

fn main() {
    println!("CODE              MENU                      PRICE");
    println!("-------------------------------------------------");
    println!("P       Poundo Yam & Edinkaiko Soup         3,200");
    println!("F       Fried Rice & Chicken                3,000");
    println!("A       Amala & Ewedu Soup                  2,500");
    println!("E       Eba & Egusi Soup                    2,000");
    println!("W       White Rice & Stew                   2,500");

    println!("Enter the code of the meal you would like to purchase");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Not a valid string");
    let code = code.trim().to_lowercase();
    
    println!("Enter the quantity of the meal you would like to purchase");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Not a valid string");
    let qty:f32 = qty.trim().parse().expect("Not a valid number");

    if code == "p"{
        let price = 3200.0;
        let total = price * qty;
        if total > 10000.0{
        let totalorder = total - (total * 0.05);
        println!("Your total charges is {}",totalorder);
      }
        else {
        println!("Your total charges is {}",total);
       }
    }

    else if code == "f"{
        let price = 3000.0;
        let total = price * qty;
        if total > 10000.0{
        let totalorder = total - (total * 0.05);
        println!("Your total charges is {}",totalorder);
        }
        else {
        println!("Your total charges is {}",total);
        }
    }

    else if code == "a"{
        let price = 2500.0;
        let total = price * qty;
        if total > 10000.0{
        let totalorder = total - (total * 0.05);
        println!("Your total charges is {}",totalorder);
        }
        else {
        println!("Your total charges is {}",total);
        }
    }

    else if code == "e"{
        let price = 2000.0;
        let total = price * qty;
        if total > 10000.0{
        let totalorder = total - (total * 0.05);
        println!("Your total charges is {}",totalorder);
        }
        else {
        println!("Your total charges is {}",total);
        }
    }

    else if code == "w"{
        let price = 2500.0;
        let total = price * qty;
        if total > 10000.0{
        let totalorder = total - (total * 0.05);
        println!("Your total charges is {}",totalorder);
        }
        else {
        println!("Your total charges is {}",total);
        }
    }

    else {
        println!("Invalid code entered!");
    }

}

