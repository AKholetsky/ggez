//! Basic hello world example.

extern crate cgmath;
extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::filesystem;
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use std::env;
use std::path;

// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    text: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));

        let s = MainState { 
            frames: 0,
            text
        };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Drawables are drawn from their top-left corner.
        let offset = self.frames as f32 / 10.0;
        let dest_point = cgmath::Point2::new(offset, offset);
        // let dest_point = cgmath::Point2::new(0.5, -0.5);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {
    let c = conf::Conf {
        window_setup: conf::WindowSetup {
            samples: conf::NumSamples::Eight,
            ..Default::default()
        },
        ..Default::default()
    };
    let (ctx, event_loop) = &mut Context::load_from_conf("helloworld", "ggez", c)?;

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(ctx, &path, true);
    }

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
