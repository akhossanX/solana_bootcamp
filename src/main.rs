fn main() {
    println!("Hello, world!");
}


fn fizz_buzz() {
	let mut fizz_buzz_counter:i16 = 0;

	for count in 1..=301 {
		if count % 3 == 0 {
			println!("fizz");
		} else if count % 5 == 0 {
			println!("buzz");
		} else {
			println!("fizz buzz");
			fizz_buzz_counter += 1;
		}
	}
	println!("{}", fizz_buzz_counter);
}
