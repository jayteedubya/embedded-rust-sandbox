# Programming a BBC micro bit with rust

## Resources

- Micro bit data sheet -- https://tech.microbit.org/hardware/
- Rust platform support listing -- https://doc.rust-lang.org/nightly/rustc/platform-support.html


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


