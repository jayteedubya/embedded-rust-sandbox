#![no_main]
#![no_std]

use cortex_m::asm::nop;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
//use panic_halt as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;


#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    // set the columns low to select them, then select the leds to light up with rows.
    board.display_pins.col1.set_low().unwrap();
    board.display_pins.col2.set_low().unwrap();
    board.display_pins.col3.set_low().unwrap();
    board.display_pins.col4.set_low().unwrap();
    // put the rows in vars, otherwise it doesn;t work. not sure why yet.
    let mut row1 = board.display_pins.row1;
    let mut row2 = board.display_pins.row2;

    loop {
        row1.set_low().unwrap();
        row2.set_low().unwrap();
        rprintln!("Dark!");
        timer.delay_ms(1_000_u32);
        row1.set_high().unwrap();
        row2.set_high().unwrap();
        rprintln!("Light!");
        timer.delay_ms(1_000_u32);
    }
}

use core::panic::PanicInfo;
#[panic_handler]
unsafe fn panic_handler(_i: &PanicInfo) -> ! {
    rprintln!("FUCK! ");
    loop {

    }
}
