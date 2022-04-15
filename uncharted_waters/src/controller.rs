use piston::input;
use piston::input::{Input, Key, Button, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonEvent, Motion};
use piston_window::*;


pub fn on_input(app: &mut App, inp: Input) {
	match inp {
		Input::Button(but) => {
			match but.state {
				ButtonState::Press => {
					println!("{:#?}", but);
					match but.button {
						Button::Keyboard(Key::Up) => {
							app.up_d = true;
						}
						Button::Keyboard(Key::Down) => {
							app.down_d = true;
						}
						Button::Keyboard(Key::Left) => {
							app.left_d = true;
						}
						Button::Keyboard(Key::Right) => {
							app.right_d = true;
						}
						Button::Mouse(MouseButton::Left) => {
						//	app.player.shoot_cannon(self.cursor);
						}
						_ => {}
					}
				}
				ButtonState::Release => {
					match but.button {
						Button::Keyboard(Key::Up) => {
							app.up_d = false;
						}
						Button::Keyboard(Key::Down) => {
							app.down_d = false;
						}
						Button::Keyboard(Key::Left) => {
							app.left_d = false;
						}
						Button::Keyboard(Key::Right) => {
							app.right_d = false;
						}
						_ => {}
					}
				}
			}				
		}
		_ => {}
 	}
}

pub fn sub () {
	println!("{}", "teste");
}
