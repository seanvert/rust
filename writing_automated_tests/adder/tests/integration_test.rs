use adder::*;
mod common;

#[test]
fn plus_two() {
	common::setup();
    assert_eq!(4, adder::add_two(2));
}



#[test]
fn larger_can_hold_smaller() {
	let larger = Rectangle {
		width: 8,
		height: 7,
	};
	let smaller = Rectangle {
		width: 5,
		height: 1,
	};

	assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
	let larger = Rectangle {
		width: 8,
		height: 7,
	};
	let smaller = Rectangle {
		width: 5,
		height: 1,
	};

	assert!(!smaller.can_hold(&larger));
}


#[test]
fn exploration() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn another() {
	panic!("make this test fail");
}

#[test]
fn it_adds_two() {
	assert_eq!(4, add_two(2));
}

#[test]
fn greeting_contains_name() {
	let result = greeting("Carol");
	assert!(result.contains("Carol"),
			"Greeting did not contain name, value was `{}`",
			result);
}

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
	Guess::new(200);
}

