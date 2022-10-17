use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

use std::cmp::Ordering;
use rand::Rng;
fn main() {
	{
	// panic!("crash and burn!");
	}
	{
		// let v = vec![1, 2, 3];

		// v[99];
	}

	{
		// recoverable error with result
		enum Result<T, E> {
			Ok(T),
			Err(E),
		}

		let greeting_file_result = File::open("hello.txt");
		// matching on different errors
		let greeting_file = match greeting_file_result {
			Ok(file) => file,
			Err(error) => match error.kind() {
				ErrorKind::NotFound => match File::create("hello.txt") {
					Ok(fc) => fc,
					Err(e) => panic!("Problem creating the file: {:?}", e),
				},
				other_error => {
					panic!("Problem opening the file: {:?}", other_error);
				}
			}

		};
	}

	{
		// alternatives to using match with Result<T, E>
		let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
			if error.kind() == ErrorKind::NotFound {
				File::create("hello.txt").unwrap_or_else(|error| {
					panic!("Problem creating the file: {:?}", error)
				})
			} else {
				panic!("Problem opening the file: {:?}", error)
			}
		});
	}

	{
		// shortcuts for Panic on Error: unwrap and expect
		let greeting_file = File::open("hello.txt").unwrap();
		let greeting_file_expect = File::open("hello.txt")
			.expect("hello.txt should be included in this project");
	}

	{
		// propagating errors
		fn read_username_from_file() -> Result<String, io::Error> {
			let username_file_result = File::open("hello.txt");

			let mut username_file = match username_file_result {
				Ok(file) => file,
				Err(e) => return Err(e),
			};

			let mut username = String::new();

			match username_file.read_to_string(&mut username) {
				Ok(_) => Ok(username),
				Err(e) => Err(e),
			}
		}
	}

	{
		// a shortcut for propagating error the ? operator
		fn read_username_from_file() -> Result<String, io::Error> {
			let mut username_file = File::open("hello.txt")?;
			let mut username = String::new();
			username_file.read_to_string(&mut username)?;
			Ok(username)
		}
	}

	{
		// creating custom types for validation
		pub struct Guess {
			value: i32,
		}

		impl Guess {
			pub fn new(value: i32) -> Guess {
				if value < 1 || value > 100 {
					panic!("Guess value must be between 1 and 100, got {}.", value);
				}

				Guess { value }
			}

			pub fn value(&self) -> i32 {
				self.value
			}
		}
		
		println!("Guess the number!", );
		let secret_number = rand::thread_rng().gen_range(1..101);
		println!("The secret number is: {}", secret_number);
		loop {
			let mut guess = String::new();
			io::stdin().read_line(&mut guess)
				.expect("Failed to read line");
			let guess: i32 = match guess.trim().parse() {
				Ok(num) => num,
				Err(_) => continue,
			};

			if guess < 1 || guess > 100 {
				println!("The secret number will be between 1 and 100");
				continue;
			}
			match guess.cmp(&secret_number) {
				Ordering::Less => println!("Too small!"),
				Ordering::Greater => println!("Too big!"),
				Ordering::Equal => {
					println!("You win!");
					break;
				}
				
			}
			
		}

	}
}
