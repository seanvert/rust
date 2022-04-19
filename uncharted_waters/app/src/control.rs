pub mod control {
	#[derive(Debug)]
	pub struct Control {
		up_d: bool, down_d: bool, left_d: bool, right_d: bool,
	}

	impl Control {
		pub fn new () -> Control {
			Control {
				up_d: false,
				down_d: false,
				left_d: false,
				right_d: false,
			}
		}

	}
}
