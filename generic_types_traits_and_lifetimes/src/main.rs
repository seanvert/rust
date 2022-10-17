use aggregator::{Summary, Tweet, NewsArticle, notify};

fn largest (list: &[i32]) -> &i32 {
	let mut largest = &list[0];

	for number in list {
		if number > largest {
			largest = number;
		}
	}

	largest
}

fn main() {
	{
		// removing duplication by extracting a function
		let number_list = vec![34, 50, 25, 100, 65];

		println!("The largest number is {}", largest(&number_list));

		let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

		println!("The largest number is {}", largest(&number_list));
	}
	{
		// generic data types
		fn largest_i32(list: &[i32]) -> &i32 {
			let mut largest = &list[0];

			for item in list {
				if item > largest {
					largest = item;
				}
			}

			largest
		}

		fn largest_char(list: &[char]) -> &char {
			let mut largest = &list[0];

			for item in list {
				if item > largest {
					largest = item;
				}
			}

			largest
		}

		fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
			let mut largest = &list[0];

			for item in list {
				if item > largest {
					largest = item;
				}
			}
			largest
		}
		let number_list = vec![34, 50, 25, 100, 65];

		let result = largest_i32(&number_list);
		println!("The largest number is {}", result);

		let char_list = vec!['y', 'm', 'a', 'q'];
		let result = largest_char(&char_list);
		println!("The largest char is {}", result);
	}

	{
		// in struct definitions
		struct Point<T, U> {
			x: T,
			y: U,
		}

		let integer = Point { x: 5, y: 10 };
		let float = Point { x: 1.0, y: 4.0 };
		let integer_and_float = Point { x: 5, y: 4.0 };
	}

	{
		// in method definitions
		struct Point<X1, Y1> {
			x: X1,
			y: Y1,
		}

		impl<T, U> Point<T, U> {
			fn x(&self) -> &T {
				&self.x
			}

			fn y(&self) -> &U {
				&self.y
			}

			fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
				Point {
					x: self.x,
					y: other.y,
				}
			}
		}

		impl Point<f32, f32> {
			fn distance_from_origin(&self) -> f32 {
				(self.x.powi(2) + self.y.powi(2)).sqrt()
			}
		}

		let p = Point { x: 5, y: 10 };
		
		println!("p.x = {}", p.x());
		println!("p.y = {}", p.y());

		let p1 = Point { x: 5, y: 10.4 };
		let p2 = Point { x: "Hello", y: 'c' };
		let p3 = p1.mixup(p2);

		println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
	}

	{
		// implementing a trait on a type
		let tweet = Tweet {
			username: String::from("horse_ebooks"),
			content: String::from(
				"of course, as you probably already know, people",
			),
			reply: false,
			retweet: false,
		};
		println!("1 new tweet: {}", tweet.summarize());
		// default implementations
		let news_article = NewsArticle {
			headline: String::from("Penguins win the Stanley Cup Championship"),
			location: String::from("Pittsburgh, PA, USA"),
			author: String::from("Iceburgh"),
			content: String::from(
				"The Pittsburgh Penguins once again are the best \
				 hockey team in the NHL.",
			),
		};

		println!("News article available! {}", news_article.summarize());

		notify(&news_article);
	}

	{
		// generic lifetimes in functions
		// lifetime annotations in functions signatures
		fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
			if x.len() > y.len() {
				x
			} else {
				y
			}
		}
		
		let string1 = String::from("abcd");
		let string2 = "xyz";
		let result = longest(string1.as_str(), string2);
		println!("The longest string is {}", result);

		let string1 = String::from("long string is long");
		{
			let string2 = String::from("xyz");
			let result = longest(string1.as_str(), string2.as_str());
			println!("The longest string is {}", result);
		}
	}

	{
		// thinking in terms of lifetimes
		// lifetime annotations in struct definitions
		struct ImportantExcerpt<'a> {
			part: &'a str,
		}

		impl<'a> ImportantExcerpt<'a> {
			fn level(&self) -> i32 {
				3
			}

			fn announce_and_return_part(&self, announcement: &str) -> &str {
				println!("Attention please: {}", announcement);
				self.part
			}
		}
		

		let novel = String::from("Call me Ishmael. Some years ago...");
		let first_sentence = novel.split('.').next().expect("Could not find a '.'");
		let i = ImportantExcerpt {
			part: first_sentence,
		};
	}

	{
		// the static lifetime
		let s: &'static str = "I hate a static lifetime.";
	}

	{
		// generic type parameters, trait bounds, and lifetimes together
		use std::fmt::Display;

		fn longest_with_an_announcement<'a, T> (
			x: &'a str,
			y: &'a str,
			ann: T,
		) -> &'a str
		where
			T: Display,
		{
			println!("Announcement! {}", ann);
			if x.len() > y.len() {
				x
			} else {
				y
			}
		}
	}
}
