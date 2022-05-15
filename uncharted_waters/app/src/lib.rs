extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate gfx_graphics;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx;
use graphics::*;

mod object;
mod view;
mod controller;
mod model;

pub mod app {
    pub use crate::object::object::Object;
    pub use crate::view::view::render;
    pub use crate::controller::control;
    pub use crate::model::model::update;
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

    pub struct App {
	rotation: f64,
	cursor: [f64; 2],
	controls: control::Control,
	pub player: Object,
	window_size: [f64; 2],
    }

    impl App {
	pub fn new() -> App {
	    App { rotation: 0.0,
		  cursor: [0.0, 0.0],
		  player: Object::new(),
		  controls: control::Control::new(),
		  window_size: [0.0, 0.0],
	    }
	}

	pub fn run(&mut self) {
	    let opengl = OpenGL::V3_2;
	    let mut window: PistonWindow = WindowSettings::new("spinning-square",
							       [1366, 768])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.resizable(true)
		.fullscreen(true)
		.build()
		.unwrap();
	    window.set_max_fps(30);
	    let mut events = Events::new(EventSettings::new());
	    println!("{} {}", window.size().width, window.size().height);
	    while let Some(mut e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
		    // TODO ajeitar o path disso
		    crate::view::view::render(self, &args, &mut window, &mut e);
		}
		if let Some(args) = e.update_args() {
		    crate::model::model::update(&mut self.player, &mut self.controls, &args);
		}
		if let Some(inp) = e.button_args() {
		    self.controls.on_input(piston::Input::Button(inp));
		}
		if let Some(coordenadas) = e.mouse_cursor_args() {
		    //					app.on_mouse(coordenadas);
		}
		if let Some(focus) = e.focus_args() {
		    println!("{:?}", focus);
		}
		if let Some(args) = e.resize_args() {
		    //					app.window_size = args.window_size;
		    println!("{:?}", args);
		}
	    }
	    let view = crate::view::view::teste {t: 0.0};
	    let model = crate::model::model::teste {t: 0.0};
	    println!("{}", "asd");
	}

	pub fn test(&mut self) {
	    println!("{:#?}", self.controls);
	    // self.up_d = true;
	}
    }
}
