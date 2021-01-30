# Simple Console abstraction RusPiRo crate

This crate provides a console abstraction to enable string output to a configurable output channel. It also provides the convinient macros (``print!`` and ``println!``) to output text that are usually not  available in ``[no_std]`` environments. However this crate also provide macros to indicate the severity of the message that shall be printed. Those are ``info!``, ``warn!`` and ``error!``.

![CI](https://github.com/RusPiRo/ruspiro-console/workflows/CI/badge.svg?branch=development)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-console.svg)](https://crates.io/crates/ruspiro-console)
[![Documentation](https://docs.rs/ruspiro-console/badge.svg)](https://docs.rs/ruspiro-console)
[![License](https://img.shields.io/crates/l/ruspiro-console.svg)](https://github.com/RusPiRo/ruspiro-console#license)

## Dependencies

This crate uses macros to provide formatted strings. This formatting requires a memory allocator to be present (as part of the ``alloc`` crate). So when using this crate provide an allocator such as ``ruspiro_allocator``.

## Usage

To use the crate just add the following dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-console = "||VERSION||"
```

Once the console crate is available the common macros used to output strings ``print!`` and ``println`` could be used.
However, without actually setting a console output those statements will not write any data anywhere:

```rust
use ruspiro_console::*;

fn demo() {
    let num: u32 = 10;
    println!("This is some text with a number: {}", num);
}
```

To actually set an active output channel you need to provide a structure that implements the ``core::fmt::Write`` trait. This
for example is done in the Uart like so:

```rust
impl core::fmt::Write for Uart1 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.send_string(s);

        Ok(())
    }
}
```

If this trait has been implemented this structure can be used as actual console. To use it there should be the following
code written at the earliest possible point in the main crate of the binary (e.g. the kernel)

```rust
use ruspiro_console::*;
use ruspiro_uart::*; // as we demonstrate the usage with the Uart.

fn demo() {
    let mut uart = Uart1::new(); // create a new uart struct
    if uart.initialize(250_000_000, 115_200).is_ok() { // initialize the Uart with fixed core rate and baud rate
        CONSOLE.with_mut(|cons| cons.replace(uart)); // from this point CONSOLE takes ownership of Uart
        // uncommenting the following line will give compiler error as uart is moved
        // uart.send_string("I'm assigned to a console");
    }

    // if everything went fine uart should be assigned to the console for generic output
    println!("Console is ready and display's through uart");
}
```

You can also use the console crate to initialize logging to the Uart when using the `info!`, `warn!` or `error!` macros from the [log](https://crates.io/crates/log).

```rust
use ruspiro_console::*;
use ruspiro_uart::*; // as we demonstrate the usage with the Uart.

fn demo() {
    let mut uart = Uart1::new(); // create a new uart struct
    if uart.initialize(250_000_000, 115_200).is_ok() { // initialize the Uart with fixed core rate and baud rate
        init_logger(LevelFilter::Error, uart); // from this point the logger takes ownership of uart
    }

    // if everything went fine uart should be assigned to the console for generic output
    println!("Console is ready and display's through uart");
}
```

> !HINT!
> As the `Write` trait requires the structure to be mutable when writing to the output channel the console operations are blocking operations -> thus requiring atomic operations to be possible -> thus requiring the *MMU* to be activated before using the console abstraction is possible. Otherwise execution will *hang* as atomics are not able to be processed by the CPU.

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
