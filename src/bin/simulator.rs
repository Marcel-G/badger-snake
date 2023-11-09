use std::{convert::Infallible, thread, time::Duration};

use badger_snake::{Direction, Game};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

const SCREEN_WIDTH: u32 = 296;
const SCREEN_HEIGHT: u32 = 128;

const GAME_HEIGHT: u32 = SCREEN_HEIGHT / 8;
const GAME_WIDTH: u32 = SCREEN_WIDTH / 8;
const SNAKE_MAX_LENGTH: usize = (GAME_HEIGHT as usize) * (GAME_WIDTH as usize);

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    let mut window = Window::new(
        "Click to move circle",
        &OutputSettingsBuilder::new()
            .scale(2)
            .theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue)
            .build(),
    );

    let mut game = Game::<SNAKE_MAX_LENGTH>::new((GAME_WIDTH, GAME_HEIGHT));

    'running: loop {
        game.update();

        display.clear(BinaryColor::On)?;

        let _ = game.draw(&mut display);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::W => game.handle_input(Direction::Up),
                    Keycode::A => game.handle_input(Direction::Left),
                    Keycode::S => game.handle_input(Direction::Down),
                    Keycode::D => game.handle_input(Direction::Right),
                    _ => {}
                },
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
