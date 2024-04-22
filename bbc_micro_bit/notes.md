# Programming a BBC micro bit with rust

## Resources

- the video I followed -- https://www.youtube.com/watch?v=TOAynddiu5M
- Micro bit data sheet -- https://tech.microbit.org/hardware/
- Rust platform support listing -- https://doc.rust-lang.org/nightly/rustc/platform-support.html
- cortex-m-rt crate -- https://crates.io/crates/cortex-m-rt
- cargo embed crate -- https://crates.io/crates/cargo-embed
- github with some good resources -- https://github.com/rust-embedded/discovery/tree/master/microbit
- rtt-target crate -- https://crates.io/crates/rtt-target
- cortex-m crate -- https://crates.io/crates/cortex-m


## Cross Compiling
Obviously trying to compile rust code on your host system will result in a binary that runs on your host system but will not run on your micro controller.
To cross comile, we must specify a "target triple" to tell the compiler what to compile for. The syntax is like this:

`<arch><sub>-<vendor>-<sys>-<env>`

where:

- arch = architecture: ARM, RISC V, etc
- sub = subarchitecture: v5, v6, etc
- vendor = vendor [optional]: apple, IBM, etc
- sys = system: none, win32, linux
- env = environment: eabi, gnu, elf

The first thing to determine is the architecture. This can be done by looking up the processor of the device on its data sheet. It may be a rabbit hole to actually find the architecture. Next, we need to find support for that architecture. Refer to the rust platform support listing above to see what is available. once you have done that, use `rustup target add <target_triple>` to add that target to the compiler. you can use `rustup show` to see installed targets.


## Bare Metal Programming

1. start a new project with cargo new
2. set up main file for bare metal (No OS libraries)
    - add `#![no_main]` and `#1[no_std]` to the top of the main file
3. To avoid doing everythin from scratch, we will download the cortex-m-rt crate, which will deal with memory mapping and all the low level stuff that needs to happen for our micro controller to run our program.
4. for this library, we will need to provide a `memory.x` file that specifies the start address and size for our flash memory and ram. This information can be found on the data sheet for the device and the data sheet for the microcontroller (it is linked in the BBC micro bit data sheet). NOTE: Make sure there is no trailing new line in the file, the compiler hates that.
    - NOTE: in the data sheet for the nordic micro controller on the micro bit the flash is referred to as "code" and the ram is referred to as "sram"
5. `cargo add cortex-m-rt`
6. set up your memory.x file
7. add the entry attribute from the library to your main function `#[entry]`
    - you will also need to make the function unsafe and never return, so the definition is like: `unsafe fn main() -> !`
8. will not strictly necesssary, for convenience, we can create a .cargo/config.toml to pass the necessary compiler flags every time we run `cargo run/build` also adjust the vscode settings for this compiler target so rust analyzer calms down
9. Now if we run `cargo check` we can see it wants a `panic_handler`. this get called when a `panic!` occurs. It is where your code goes to exit after a fault.
10. the panic handler uses the `#[panic_handler]` decorator and has a definition like this: `unsafe fn panic_handler(_i: &PanicInfo) -> !`
11. for our purposes, we will use the `panic_halt` crate, which halts when a panic occurs. to use it, import it in the main file with `use panic_halt as _;` 
12. we can now build the project. It does nothing right now, but it is good to find out where things end up. you can find the elf file (in the case of the BBC microbit project) in `target/thumbv7em-none-eabihf/debug/<project_name (no extension)>
13. Install llvm-tools to get info about the binary file we've created. `rustup component add llvm-tools` and `cargo install cargo-binutils`
14. run `cargo size -- -Ax` to get some basic info about the binary file we created.
15. we need to install cargo embed, but first we need to install its dependencies. on ubuntu, it is: `sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev`. For other platforms, it is listed on the crates.io package page for the crate.
16. run `cargo install cargo-embed`
17. run `cargo-embed --list-chips` to see what is available
    - there are a lot of items in that list, so probably best to grep it.
18. plug in the device
19. run `cargo-embed --chip <your chip here>`. "nRF52833_xxAA" for the micro bit
20. on ubuntu I needed to setup udev rule for the device
    - `sudo nano /etc/udev/rules.d/99-microbit.rules`
    - paste in:
    ```
    # CMSIS-DAP for microbit
    SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
    ```
    - run `sudo udevadm control --reload-rules`
    - unplug device and plug it back in
    - see where the device enumerated with `lsusb | grep -i "NXP ARM mbed"`
    - check permissions with `ls -l /dev/bus/usb/<bus>/<device>`
21. It should work. To remove the need to pass the argument every time, will will create an `Embed.toml` file in the root of the project
    - add this stuff:
    ```toml
    [default.general]
    chip = "nRF52833_xxAA"
    ```
22. run `cargo embed`. it should still work.


## Debugging (With RTT)

1. install minicom and gdb `sudo apt-get install gdb-multiarch minicom`
    - you may need to create a symlink for gdb to work. 
    `sudo ln -s /usr/bin/gdb-multiarch /usr/bin/arm-none-eabi-gdb`
2. install rtt-target and cortex-m critical single care crates with cargo. `cargo add rtt-target` and `cargo add cortex-m --features critical-section-single-core`
3. cargo embed will facilitate rtt, so we need to add some more config to Embed.toml
    - add this to the embed file
```toml
[default.rtt]
enabled = true
```
4. import and add a call to the print init macro to the top of main.
```rust
use rtt_target::{rprintln, rtt_init_print};
//...
fn main() {
    rtt_init_print!();
    //...
}
```
5. you can now use `rprintln!` throughout your code to print messages
6. run cargo embed and you will see responses coming back from the device. ctrl-c to exit

## Debugging (with GDB)
1. add this to the Embed.toml
```toml
[default.gdb]
enabled = true

[default.reset]
halt_afterwards = true
```
2. disable rtt by changing the default.rtt to this
```toml
[default.rtt]
# enabled = true
enabled = false
```
3. remove the rprintln stuff
4. in one terminal, `run cargo embed`.
5. in another terminal, run `arm-none-eabi-gdb <path_to_elf>`
6. gdb should now be running
7. in gdb run `target remote :1337` to connect to device
8. gdb is very powerful and can do many things.
    - to view registers, run `info registers`
    - add a breakpoint with `break <file_name>:<line_number>`

## Actually doing something

- Install the microbit crate from cargo. Note that there are two separate crates, one for each different version of microbit.
- Also worth noting is that you will need to include the crate in your main.rs or cargo embed will fail to compile.














