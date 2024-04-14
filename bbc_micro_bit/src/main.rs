#![no_std]
#![no_main]



use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();
    rprintln!("Howdy, I am micro bit");
    let mut x = 0;
    loop {
        rprintln!("I'm OK");
        x += 1;
        for _ in 0..x {
            nop(); // no op instruction to add delay
        }
    }
}

// use core::panic::PanicInfo;
// #[panic_handler]
// unsafe fn panic_handler(_i: &PanicInfo) -> ! {
//     loop {

//     }
// }