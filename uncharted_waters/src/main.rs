extern crate glutin_window;
extern crate graphics;
// extern crate opengl_graphics;
extern crate piston;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx;

// debug
use std::any::type_name;
use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::TextureContext;
use gfx_graphics::Flip;
use gfx_graphics::Texture;
use gfx_graphics::TextureSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Input, Key, Button, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonEvent, Motion};
use piston::input::Input::Move;
use piston::ButtonState;
use piston::window::WindowSettings;
use piston_window::*;
use graphics::*;
use graphics::rectangle::square;
use std::f64::consts::PI;
// mods
mod object;
use object::Object;


pub struct App {
	rotation: f64,
	cursor: [f64; 2],
	up_d: bool,
	down_d: bool,
	left_d: bool,
	right_d: bool,
	player: Object,
	window_size: [f64; 2],
	sprite: Option<Texture<Resources>>,
}

fn print_type_of<T>(_: &T) {
	println!("{}", std::any::type_name::<T>());
}

impl App {
	pub fn new() -> App {
		App { rotation: 0.0,
			  cursor: [0.0, 0.0],
			  player: Object::new(),
			  up_d: false, down_d: false, left_d: false, right_d: false,
			  window_size: [0.0, 0.0],
			  sprite: None,
		}
	}
	
	pub fn on_load(&mut self, window: &mut PistonWindow) {
		let assets = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("assets").unwrap();
		let ships = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Ships").unwrap();
		let ship_parts = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Ship parts").unwrap();
		// TODO pega as pastas
		//  TODO coloca os arquivos num vetor
		let ship_sprite = ships.join("ship (1).png");
		let tank_sprite = assets.join("E-100_preview.png");
		let cannon_ball_sprite = ship_parts.join("cannonBall.png");
		let mut texture_context = TextureContext {
			factory: window.factory.clone(),
			encoder: window.factory.create_command_buffer().into()
		};
		// TODO itera o vetor com as texturas
		let cannon_ball_sprite = Texture::from_path(
			&mut texture_context,
			&cannon_ball_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		let ship_sprite = Texture::from_path(
			&mut texture_context,
			&ship_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.player.set_sprite(ship_sprite);
		self.player.set_cannon_ball_sprite(cannon_ball_sprite);

		let tiles = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("Tiles").unwrap();
		let agua_sprite = tiles.join("tile_73.png");
		let agua_sprite = Texture::from_path(
			&mut texture_context,
			&agua_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.sprite = Some(agua_sprite);
	}

	pub fn on_mouse(&mut self, coordenadas: [f64; 2]) {
		self.cursor = coordenadas;
//		println!("{:#?} {}", x, y);
	}
	
	pub fn render(&mut self, args: &RenderArgs,
			  gl: &mut PistonWindow, e: &mut Event) {
		let image = Image::new().rect(square(0.0, 0.0, 1000.0));
		const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
		self.window_size = [args.window_size[0], args.window_size[1]];
		gl.draw_2d(e, |c, g, _| {
			clear(GREEN, g);
			let transform = c.transform;
			if let Some(sprite) = &self.sprite {
				image.draw(sprite, &DrawState::new_alpha(),
						   c.transform, g);
			};
			self.player.render(g, transform);
		});
	}

	pub fn on_input(&mut self, inp: Input) {
		match inp {
			Input::Button(but) => {
				match but.state {
					ButtonState::Press => {
						println!("{:#?}", but);
						match but.button {
							Button::Keyboard(Key::Up) => {
								self.up_d = true;
							}
							Button::Keyboard(Key::Down) => {
								self.down_d = true;
							}
							Button::Keyboard(Key::Left) => {
								self.left_d = true;
							}
							Button::Keyboard(Key::Right) => {
								self.right_d = true;
							}
							Button::Mouse(MouseButton::Left) => {
								self.player.shoot_cannon(self.cursor);
							}
							_ => {}
						}
					}
					ButtonState::Release => {
						match but.button {
							Button::Keyboard(Key::Up) => {
								self.up_d = false;
							}
							Button::Keyboard(Key::Down) => {
								self.down_d = false;
							}
							Button::Keyboard(Key::Left) => {
								self.left_d = false;
							}
							Button::Keyboard(Key::Right) => {
								self.right_d = false;
							}
							_ => {}
						}
					}
				}				
			}
			_ => {}
 		}
	}
	
	pub fn update(&mut self, args: &UpdateArgs) {
		let speed = 500.0;
		if self.player.no_ar {
			self.player.move_cannon_ball(1.0);
		}
		if self.up_d {
			if self.player.y > 0.0 {
				self.player.mov(0.0, -speed * args.dt);
				self.player.rot_to(PI);
			}
		}
		if self.down_d {
			if self.player.y < self.window_size[1] {
				self.player.mov(0.0, speed * args.dt);
				self.player.rot_to(0.0);
			}
		}
		if self.left_d {
			if self.player.x > 0.0 {
				self.player.mov(-speed * args.dt, 0.0);
				self.player.rot_to(PI/2.0);
			}
		}
		if self.right_d {
			if self.player.x < self.window_size[0] {
				self.player.mov(speed * args.dt, 0.0);
				self.player.rot_to(3.0*PI/2.0);
			}
		}
	}
}

fn main() {
	let opengl = OpenGL::V3_2;
	let mut window: PistonWindow = WindowSettings::new("spinning-square",
													   [1080, 1873])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.resizable(true)
		.fullscreen(true)
		.build()
		.unwrap();
	window.set_max_fps(30);
	let mut app = App::new();

	app.on_load(&mut window);
	let mut events = Events::new(EventSettings::new());
	println!("{} {}", window.size().width, window.size().height);
	while let Some(mut e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args, &mut window, &mut e);
		}
		if let Some(args) = e.update_args() {
			app.update(&args);
		}
		if let Some(inp) = e.button_args() {
			app.on_input(piston::Input::Button(inp));
		}
		if let Some(coordenadas) = e.mouse_cursor_args() {
			app.on_mouse(coordenadas);
		}
		if let Some(focus) = e.focus_args() {
			println!("{:?}", focus);
		}
		if let Some(args) = e.resize_args() {
			app.window_size = args.window_size;
			println!("{:?}", args);
		}
	}
}
