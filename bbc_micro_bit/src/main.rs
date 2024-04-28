#![no_std]
#![no_main]



use cortex_m::asm::nop;
use cortex_m_rt::entry;
// use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};


#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    rprintln!("Howdy, I am micro bit");
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