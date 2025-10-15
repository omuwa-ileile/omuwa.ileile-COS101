fn main(){
	let t: f64 = 450000.0;
	let m: f64 = 1500000.0;
	let h: f64 = 750000.0;
	let d: f64 = 2850000.0;
	let a: f64 = 250000.0;
    let s = (t + m + h + d + a);
    let q = (2.0 + 1.0 + 3.0 + 3.0 + 1.0);
    let v = (s / q);
    println!("The sum of the sales record is {}" , s);
    println!("The average of the sales record is {}" , v);
}