struct User {
    active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someoneusername123"),
		active: true,
		sign_in_count: 1,
	};
	println!("{}", user1.email);
	user1.email = String::from("outroemail@example.com");
	println!("{}", user1.email);
    println!("Hello, world!");
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
	User {
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
}
