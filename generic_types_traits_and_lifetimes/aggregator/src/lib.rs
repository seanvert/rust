pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// traits: defining shared behavior
pub trait Summary {
	fn summarize_author(&self) -> String;
	
	fn summarize(&self) -> String {
		format!("(Read more {}...)", self.summarize_author())
	}
}

// implementing a trait on a type
pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

// traits as parameters
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}

pub fn notify_verbose<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}

impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("{}", self.author)
	}
	
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
	
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// using trait bounds to conditionally implement methods

use std::fmt::Display;

struct Pair<T> {
	x: T,
	y: T,
}

impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}
