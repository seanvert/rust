pub mod view {
    use graphics::*;
    use crate::app::App;
    use piston_window::RenderArgs; // TODO checar se é isto mesmo ou se é o outro
    use piston_window::{PistonWindow, clear, Image, Event, rectangle::square};
    
    pub fn render (app: &mut App, args: &RenderArgs,
		   gl: &mut PistonWindow, e: &mut Event,
		   // FIXME passar um iterável com a lista de objetos
		   // atualmente está passando só as coordenadas de um objeto só
		   x: f64,
		   y: f64
    ) {
	let image = Image::new().rect(square(0.0, 0.0, 1000.0));
	const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
	let square = rectangle::square(0.0, 0.0, 100.0);
	let red = [1.0, 0.0, 0.0, 1.0];
	//		self.window_size = [args.window_size[0], args.window_size[1]];
	gl.draw_2d(e, |c, g, _| {
	    clear(GREEN, g);
	    let transform = c.transform;
	    // TODO fazer um loop nos objetos e renderizar as coordenadas e as sprites
	    rectangle(red, square, transform.trans(x, y), g);
	    // TODO aqui acho que vai um iteravel de objetos que estão na cena
	    // XXX dá pra eu passar sprite, posição e rotação
	    // FIXME e esse iteravel vai ficar onde: acho que no model
	    // if let Some(sprite) = &self.sprite {
	    // 	image.draw(sprite, &DrawState::new_alpha(),
	    // 			   c.transform, g);
	    // };
	    // self.player.render(g, transform);
	});
    }
    pub struct teste {
	pub t: f64,
    }
}
