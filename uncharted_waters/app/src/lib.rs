#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
mod front_of_house;
pub use crate::front_of_house::hosting;
mod object;
use crate::object::object::Object;

pub fn testeasd() {
	hosting::add_to_waitlist();
	Object::new();
}
// pub use crate::object;
pub mod app {
	pub struct App {
		rotation: f64,
		cursor: [f64; 2],
		up_d: bool,
		down_d: bool,
		left_d: bool,
		right_d: bool,
//		teste: teste,
//		player: Object,
		window_size: [f64; 2],
	}

	impl App {
		pub fn new() -> App {

			App { rotation: 0.0,
				  cursor: [0.0, 0.0],
//				  player: Object::new(),
				   up_d: false, down_d: false, left_d: false, right_d: false,

				  window_size: [0.0, 0.0],
//				  sprite: None,
			}
		}
	}
}
