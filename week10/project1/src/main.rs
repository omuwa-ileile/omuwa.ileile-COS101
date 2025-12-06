struct Laptop {
   brand: String,
   price: u32,
}

impl Laptop {
    fn cost(&self, qty: u32) -> u32{
        self.price * qty
    }
}

fn main() {
    let laptops = vec![
        Laptop {brand:"HP".to_string(), price: 650_000},
        Laptop {brand:"IBM".to_string(), price: 755_000},
        Laptop {brand:"Toshiba".to_string(), price: 550_000},
        Laptop {brand:"Dell".to_string(), price: 850_000},
       ];

       let qty = 3;
       let mut total = 0;

       for item in &laptops {
       let total_cost = item.cost(qty);
       println!("{} * {} laptops = {}",item.brand, qty, total_cost);
       total += total_cost;
       }

       println!("\nTotal cost for buying 3 of each brand = {}", total);
}
