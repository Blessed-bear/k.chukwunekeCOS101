 fn main() {
 	// multiply the quantity by the amounts to get the total amounts
	let t:f64 = 450_000.0 * 2.0;
	let m:f64 = 1_500_000.0 * 1.0;
	let h:f64 = 750_000.0 * 3.0;
	let d:f64 = 3_850_000.0 * 3.0;
	let a:f64 = 250_000.0 * 1.0; 

	//sum of all the amounts 
	let s:f64 = t + m + h + d + a;
	println!("The sum of all the amounts is {}",s ); 
	//  average of the amounts
	let a:f64 = s / 10.0; 
	println!("The average of the amounts is {}",a );

}