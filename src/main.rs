#![no_main]
#![no_std]

use core::time;

use microbit::{display, hal::gpio::Disconnected, pac};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use embedded_hal::digital::{InputPin, OutputPin};
#[rustfmt::skip]
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer, gpio},
};

use hsv::{Hsv, Rgb};

struct RgbDisplay {
    // What tick of the frame are we currently on?
    // Setting to 0 starts a new frame.
    tick: u32,
    // What ticks should R, G, B LEDs turn off at?
    schedule: [u32; 3],
    // Schedule to start at next frame.
    next_schedule: Option<[u32; 3]>,
    // R, G, and B pins.
    rgb_pins: [gpio::Pin<gpio::Output<gpio::PushPull>>; 3],
    // Timer used to reach next tick.
    timer0: Timer<pac::TIMER0>,
}

impl RgbDisplay {
    fn new(rgb_pins: [gpio::Pin<gpio::Disconnected>; 3], timer0: Timer<pac::TIMER0>) -> Self {
        let [r, g, b] = rgb_pins;
        let r = r.into_push_pull_output(gpio::Level::High);
        let g = g.into_push_pull_output(gpio::Level::High);
        let b = b.into_push_pull_output(gpio::Level::High);
        let rgb_pins = [r, g, b];
        Self {
            tick: 0,
            schedule: [0; 3],
            next_schedule: None,
            rgb_pins,
            timer0,
        }
    }

    /// Set up a new schedule, to be started next frame.
    fn set(&mut self, hsv: &Hsv) {
        let rgb = hsv.to_rgb();
        let r = (rgb.r * 100.0) as u32;
        let g = (rgb.g * 100.0) as u32;
        let b = (rgb.b * 100.0) as u32;
        self.next_schedule = Some([r, g, b]);
    }

    /// Take the next frame update step. Called at startup
    /// and then from the timer interrupt handler.
    fn step(&mut self) {
        todo!()
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut display = Display::new(board.display_pins);

    let mut timer0 = Timer::new(board.TIMER0);
    let mut timer1 = Timer::new(board.TIMER1);

    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0);}
    pac::NVIC::unpend(pac::Interrupt::TIMER0);

    let display_h = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ];
    let display_s = [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ];
    let display_v = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let mut state = 0;
    loop {
        match (button_a.is_low().unwrap(), button_b.is_low().unwrap()) {
            (true, false) => state = (state + 3 - 1) % 3,
            (false, true) => state = (state + 1) % 3,
            _ => (),
        };
        match state {
            0 => display.show(&mut timer1, display_h, 100),
            1 => display.show(&mut timer1, display_s, 100),
            2 => display.show(&mut timer1, display_v, 100),
            _ => panic!(),
        }
    }
}
