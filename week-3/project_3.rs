fn main() {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	
	//finding depreciation
	let c = 1.0 - (r/100.0);
	let a = c * c * c;
	let b = p - a;
	println!("The depreciation for the TV sales {}", b);

}