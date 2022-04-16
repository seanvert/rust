use restaurant::eat_at_restaurant;
use restaurant::hosting;
fn main() {
	hosting::add_to_waitlist();
	eat_at_restaurant();
    println!("Hello, world!");
}
