fn main() {
    println!("Hello, world!");
	let mut s = String::from("Hello");
	s.push_str(", world!");
	println!("{}", s);

	// clone para copiar o conteudo de ponteiros
	// a atribuição não vai por referência se usarmos esse método
	let s1 = String::from("hello");
	let mut s2 = s1.clone();
	s2.push_str("teste");
	println!("s1 {} s2 {}", s1, s2);

	let mut s = String::from("Hello");

	s = takes_ownership(s);
	takes_ownership(s);
	let x = 3;

	makes_copy(x);
	makes_copy(x);

	let s1 = gives_ownership();

	let s2 = String::from("hello");
	println!("{} {}", s1, s2);
	let s3 = takes_and_gives_back(s2); // s2 sai do escopo
	println!("{} {}", s1, s3);

	let (s4, len) = calculate_length(s3); // s3 sai do escopo

	println!("The length of '{}' is {}", s4, len);
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}

fn gives_ownership() -> String {
	let some_string = String::from("yours");
	some_string
}

fn takes_and_gives_back(string: String) -> String {
	string
}

fn takes_ownership(string: String) -> String {
	println!("{}", string);
	string
}

fn makes_copy(integer: i32) {
	println!("{}", integer);
}
