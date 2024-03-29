pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
	format!("Hello {}!", name)
}

pub struct Guess {
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 {
			panic!("Guess value must be greater than or equal to 1, got {}.", value);
		} else if value > 100 {
			panic!("Guess value must be less than or equal to 100, got {}.", value);
		}

		Guess { value }
	}
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
	pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool  {
		self.width > other.width && self.height > other.height
	}
}

pub fn add_two(a: i32) -> i32 {
	a + 2
}


