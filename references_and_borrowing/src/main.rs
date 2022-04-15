fn main() {
	let s1 = String::from("hello");

	let len = calculate_length(&s1);

	println!("The length of '{}' is {}.", s1, len);

	let mut s = String::from("hello");
{
	let r1 = &mut s;
	change(r1); // só dá pra usar uma por vez
	change(r1);
}
	let r2 = &mut s;
	change(r2);
	println!("{}", s);
}

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
	s.len()
}
