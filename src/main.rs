use ggez::{
    Context,
    GameResult,
    glam::*,
    graphics::{self, Color, TextFragment, PxScale},
    event::{self, EventHandler},
    input::keyboard::KeyCode, audio::{self, Source, SoundSource}
};
use min_max::min;

use std::{ path, env, time::{Instant, Duration} };

mod gol;

use gol::gol::{
	WIDTH,
	HEIGHT,
	FPS_CAP,
	MILLIS
};

const WWIDTH: usize = 800;
const WHEIGHT: usize = 600;

const CELL_SIZE_W: f32 = WWIDTH as f32 / WIDTH as f32;
const CELL_SIZE_H: f32 = WHEIGHT as f32 / HEIGHT as f32;

const CELL_SIZE: f32 = if CELL_SIZE_W > CELL_SIZE_H { CELL_SIZE_H } else { CELL_SIZE_W };

fn main() {
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("pong", "Maarten van Keulen").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build().expect("Could not build context");
	
	let my_game = MyGame::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
	buffer: [[bool;WIDTH];HEIGHT],
	lastUpdate: Instant
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
		let mut buffer = [[false;WIDTH];HEIGHT];
		gol::gol::load_from_file(&mut buffer);

		MyGame {
			buffer,
			lastUpdate: Instant::now()
		}
	}
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
		if Instant::now().duration_since(self.lastUpdate) > MILLIS {
			gol::gol::tick(&mut self.buffer);
			self.lastUpdate = Instant::now();
		}
		
		Ok(())
	}
	
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
		let buffer = self.buffer;
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let alive = buffer[y][x];

				if alive {
					let pa = &mut graphics::MeshBuilder::new();
					pa.rectangle(
						graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
						graphics::Rect::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE),
						graphics::Color::new(1.0, 1.0, 1.0, 1.0),
					)?;

					let mut cell = graphics::Mesh::from_data(ctx, pa.build());
					canvas.draw(&mut cell, graphics::DrawParam::new());
				}
			}
		}

        canvas.finish(ctx)
	}
}
