fn main() {
	let mut s = String::from("Hello, world!");

	let hello = &s[0..5];
	let world = &s[7..12];
	println!("{}{}", hello, world);
	println!("{}", first_word(&s));
}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}

	&s
}
