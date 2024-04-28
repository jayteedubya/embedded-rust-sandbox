#![no_main]
#![no_std]

use cortex_m::asm::nop;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
//use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;
use embedded_hal::digital::OutputPin;


#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    rprintln!("Howdy, I am micro bit");
    
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_high().unwrap();
    board.display_pins.row1.set_high().unwrap();
    loop {
        rprintln!("I'm OK");
        for _ in 0..10000 {
            nop(); // no op instruction to add delay
        }
    }
}

use core::panic::PanicInfo;
#[panic_handler]
unsafe fn panic_handler(_i: &PanicInfo) -> ! {
    rprintln!("FUCK! ");
    loop {

    }
}
