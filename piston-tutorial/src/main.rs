extern crate piston_window;

use piston_window::*;

struct Game {
	rotation: f64,
}

impl game {
	fn new() -> Game {
		Game { rotation: 0.0 }
	}

	fn on_update(&mut self, upd: UpdateArgs) {
		self.rotation += 3.0 * upd.dt;
	}

	fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {

		e.draw_2d(&e, |c, g, _| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			let center = c.transform.trans(300.0, 300.0);
			let square = rectangle::square(0.0, 0.0, 100.0);
			let red = [1.0, 0.0, 0.0, 1.0];
			rectangle(red, square, center.trans(-50.0, -50.0), g);
		});
	}
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("piston tutorial!", [600, 600])
        .exit_on_esc(true).build().unwrap();
	while let Some(e) = window.next() {

	}

}
