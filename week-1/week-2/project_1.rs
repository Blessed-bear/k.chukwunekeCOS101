fn main() {
	let p:f64 = 520_000_000.00;
	let t:f64 = 5.0;
	let r:f64 = 10.0;

	// simple interest
	let a:f64 = p * (1.0 + (r / 100.0)).powf(t); 
	// compound interest
	let ci:f64 = a - p;
	println!("The compound interest is {}",ci );
}	