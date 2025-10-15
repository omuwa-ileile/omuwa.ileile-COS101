fn main(){
	let t:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
	let a = t * ((1.0 - (r / 100.0)).powf(n));
	println!("The value of the tv after 3 years is {}" , a)
}