fn main() {
	let t:f64 = 420_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;

	//sum
	let s = t + m + h + d + a;
	println!("Sum is {}", s );

	//Average
	let av = s / 5.0;
	println!("Average of the sale record {}", av);
	
}
   
