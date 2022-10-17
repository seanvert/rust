use std::collections::HashMap;
fn main() {
	{
		// accessing values in a hash map
		let mut scores = HashMap::new();
		scores.insert(String::from("Blue"), 10);
		scores.insert(String::from("Yellow"), 50);

		let team_name = String::from("Blue");
		let score = scores.get(&team_name).copied().unwrap_or(0);
		println!("{}", score);

		for (key, value) in &scores {
			println!("{}: {}", key, value);
		}
	}

	{
		// hash maps and ownership
		let field_name = String::from("Favorite color");
		let field_value = String::from("Blue");

		let mut map = HashMap::new();
		map.insert(field_name, field_value);
	}

	{
		// adding a key and value only if a key isn't present
		let mut scores = HashMap::new();
		scores.insert(String::from("Blue"), 10);

		scores.entry(String::from("Yellow")).or_insert(50);
		scores.entry(String::from("Blue")).or_insert(50);

		println!("adding values only if a key isn't present {:?}", scores);
	}

	{
		// updating a value based on the old value
		let text = "hello world wonderful world";

		let mut map = HashMap::new();

		for word in text.split_whitespace() {
			let count = map.entry(word).or_insert(0);
			*count += 1;
		}
		println!("{:?}", map);
	}
	
	{
		enum SpreadsheetCell {
			Int(i32),
			Float(f64),
			Text(String),
		}

		let row = vec![
			SpreadsheetCell::Int(3),
			SpreadsheetCell::Text(String::from("blue")),
			SpreadsheetCell::Float(10.23),
		];
	}
}
