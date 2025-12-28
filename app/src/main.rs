use ggez::{Context, GameResult, conf, event, graphics};
use graphics::{Color};

use rustamatix::{self, Animation, AnimationParams};

const WIN_DIM: (f32, f32) = (1000.0, 1000.0);
const BG_COLOUR: Color = Color::new(0.1, 0.2, 0.3, 1.0);

struct Game {
    animation: Animation
}

impl Game {
    fn new(animation: Animation) -> Game {
        Game { animation: animation}
    }
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.animation.add_frame_time(_context.time.delta().as_secs_f32());

        self.animation.next_frame();

        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(_context, BG_COLOUR);

        canvas.draw(self.animation.spritesheet(), self.animation.draw_params());

        canvas.finish(_context)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("test", "hayden")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!").vsync(true))
        .window_mode(conf::WindowMode::default().dimensions(WIN_DIM.0, WIN_DIM.1))
        .add_resource_path(std::path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;

    let image = graphics::Image::from_path(&mut context, "/bar-sheet.png")?;

    let ani_params = AnimationParams::new((436.0, 436.0), (128.0, 128.0), 15.0, 1.0 / 15.0);

    let animation = Animation::new(image, ani_params);
    // Create the game state
    let game = Game::new(animation);
    // Run the main event loop
    event::run(context, event_loop, game)
}