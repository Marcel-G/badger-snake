//! # Badger2040 Blinky Example
//!
//! Blinks the activity LED on a badger2040 board, using an RP2040 Timer in Count-down mode.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]

use defmt_rtt as _;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::Drawable;
use embedded_hal::timer::CountDown;
use fugit::MicrosDurationU32;
use fugit::RateExtU32;

// The macro for our start-up function
use pimoroni_badger2040::entry;

// GPIO traits
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
// use panic_halt as _;
use panic_probe as _;

use pimoroni_badger2040::hal::Clock;
use pimoroni_badger2040::hal::Timer;
// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use pimoroni_badger2040::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use badger_snake::Direction;
use badger_snake::Game;
use badger_snake::Update;
use pimoroni_badger2040::hal;
use uc8151::UpdateRegion;
use uc8151::HEIGHT;
use uc8151::WIDTH;

const GAME_HEIGHT: u32 = HEIGHT / 8;
const GAME_WIDTH: u32 = WIDTH / 8;
const SNAKE_MAX_LENGTH: usize = (GAME_HEIGHT as usize) * (GAME_WIDTH as usize);

#[entry]
fn main() -> ! {
    let core = pac::CorePeripherals::take().unwrap();

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        pimoroni_badger2040::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = pimoroni_badger2040::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure the timer peripheral to be a CountDown timer for our blinky delay
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Set up the pins for the e-ink display
    let _spi_sclk = pins.sclk.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_mosi = pins.mosi.into_mode::<hal::gpio::FunctionSpi>();
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI0);
    let mut dc = pins.inky_dc.into_push_pull_output();
    let mut cs = pins.inky_cs_gpio.into_push_pull_output();
    let busy = pins.inky_busy.into_pull_up_input();
    let reset = pins.inky_res.into_push_pull_output();

    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        RateExtU32::MHz(1),
        &embedded_hal::spi::MODE_0,
    );

    dc.set_high().unwrap();
    cs.set_high().unwrap();

    let mut count_down = timer.count_down();
    let mut display = uc8151::Uc8151::new(spi, cs, dc, busy, reset);

    // Reset display
    display.disable();
    count_down.start(MicrosDurationU32::micros(10));
    let _ = nb::block!(count_down.wait());
    display.enable();
    count_down.start(MicrosDurationU32::micros(10));
    let _ = nb::block!(count_down.wait());
    // Wait for the screen to finish reset
    while display.is_busy() {}

    // Initialise display. Using the default LUT speed setting
    let _ = display.setup(&mut delay, uc8151::LUT::Ultrafast);

    let sw_up = pins.sw_up.into_pull_down_input();
    let sw_down = pins.sw_down.into_pull_down_input();
    let sw_left = pins.sw_a.into_pull_down_input();
    let sw_reset = pins.sw_b.into_pull_down_input();
    let sw_right = pins.sw_c.into_pull_down_input();

    let mut game = Game::<SNAKE_MAX_LENGTH>::new((GAME_WIDTH, GAME_HEIGHT));

    let _ = display.update();

    let _ = game.draw(&mut display);
    let _ = display.update();

    loop {
        if sw_up.is_high().unwrap() {
            game.handle_input(Direction::Up);
        }
        if sw_down.is_high().unwrap() {
            game.handle_input(Direction::Down);
        }
        if sw_left.is_high().unwrap() {
            game.handle_input(Direction::Left);
        }
        if sw_right.is_high().unwrap() {
            game.handle_input(Direction::Right);
        }
        if sw_reset.is_high().unwrap() {
            game.reset();
            let _ = display.clear(BinaryColor::On);
            let _ = game.draw(&mut display);
            let _ = display.update();
            while display.is_busy() {}
        }

        if display.is_busy() || game.game_over || count_down.wait().is_err() {
            continue;
        }

        count_down.start(MicrosDurationU32::millis(125));

        let update = game.update();

        let _ = display.clear(BinaryColor::On);
        let _ = game.draw(&mut display);

        match update {
            Update::Food(..) => {
                let _ = display.update();
            }
            Update::Snake(new, previous) => {
                let _ =
                    display.partial_update(UpdateRegion::new(new.x * 8, new.y * 8, 8, 8).unwrap());
                let _ = display.partial_update(
                    UpdateRegion::new(previous.x * 8, previous.y * 8, 8, 8).unwrap(),
                );
            }
            Update::None => {}
        }
    }
}
