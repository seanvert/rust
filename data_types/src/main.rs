use std::io;

fn main() {
	let a: u8 = 1;
	let b = 0;
	if let Some(a) = (a + 254).checked_add(b) {
		println!("{}", a);
	} else {
		println!("{}", "overflow");
	}

	let a = [1, 2, 3, 4, 5];

	println!("Please enter an array index.", );

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line.");

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number.");

	let element = a[index];

	println!("The value of the element at index {} is: {}", index, element);
}
