use std::io::Write;

fn main() {
   let heading = "Larger      Stout        Non-Alcoholic\n";
   let lines = "--------------------------------------------\n";
   let row1 = "33Export       Legend       Maltina\n";
   let row2 = "Desperados     Turbo King   Amstel Malta\n";
   let row3 = "Goldberg       Williams     Malta Gold\n";
   let row4 = "Gulder                      Fayrouz\n";
   let row5 = "Heineken\n";
   let row6 = "Star\n";

   let mut file = std::fs::File::create("drinks_categories.txt").expect("create failed");
   file.write_all("Nigerian Breweries Plc Drink Categories\n".as_bytes()).expect("write failed");
   file.write_all(heading.as_bytes()).expect("write failed");
   file.write_all(lines.as_bytes()).expect("write failed");
   file.write_all(row1.as_bytes()).expect("write failed");
   file.write_all(row2.as_bytes()).expect("write failed");
   file.write_all(row3.as_bytes()).expect("write failed");
   file.write_all(row4.as_bytes()).expect("write failed");
   file.write_all(row5.as_bytes()).expect("write failed");
   file.write_all(row6.as_bytes()).expect("write failed");
   println!("file created");
}
