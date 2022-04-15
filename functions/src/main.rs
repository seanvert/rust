fn main() {
	println!("Hello, world");

	another_function(5);

	print_labeled_measurement(5, 'h');

	let y = {
		let x = 2;
		x + 2
	};

	println!("O valor de y é: {}", y);

	println!("{}", five());
	println!("O valor de plus_one({}) é {}", 4, plus_one(4));
}

fn plus_one(x: i32) -> i32 {
	x + 1
}

fn five() -> i32 {
	5
}
fn another_function(x: i32) {
	println!("O valor de x é {}", x);
}

fn print_labeled_measurement(value: i32, unit: char) {
	println!("The measurement is: {}{}", value, unit);
}
