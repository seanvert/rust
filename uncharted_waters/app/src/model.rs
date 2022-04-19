pub mod model {
	// TODO fazer uma lista de objetos para iterar no render do view
	// XXX vou iterar isso nos objetos?
	 use std::f64::consts::PI;
	use piston_window::UpdateArgs;
	use crate::object::object::Object;
	use crate::controller::control::Control;
	pub fn update(obj: &mut Object, control: &Control, args: &UpdateArgs) {
		let speed = 500.0;
		// if self.player.no_ar {
		// 	self.player.move_cannon_ball(1.0);
		// }
		if control.up_d {
			// if self.player.y > 0.0 {
			obj.mov(0.0, -speed * args.dt);
			obj.rot_to(PI);
			// }
		}
		if control.down_d {
			// if self.player.y < self.window_size[1] {
			obj.mov(0.0, speed * args.dt);
			obj.rot_to(0.0);
			// }
		}
		if control.left_d {
			// if self.player.x > 0.0 {
			obj.mov(-speed * args.dt, 0.0);
			obj.rot_to(PI/2.0);
			// }
		}
		if control.right_d {
			// if self.player.x < self.window_size[0] {
			obj.mov(speed * args.dt, 0.0);
			obj.rot_to(3.0*PI/2.0);
			// }
		}
	}

	
	pub struct teste {
		pub t: f64,
	}
}
