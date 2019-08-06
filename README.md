# Simple Console abstraction RusPiRo crate

This crate provides a console abstraction to enable string output to a configurable output channel.
It also provides the convinient macros (``print!`` and ``println!``) to output text that are usually not 
available in ``[no_std]`` environments. However this crate also provide macros to indicate the severity of the 
message that shall be printed. Those are ``info!``, ``warn!`` and ``error!``.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-console.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-console)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-console.svg)](https://crates.io/crates/ruspiro-console)
[![Documentation](https://docs.rs/ruspiro-console/badge.svg)](https://docs.rs/ruspiro-console)
[![License](https://img.shields.io/crates/l/ruspiro-console.svg)](https://github.com/RusPiRo/ruspiro-console#license)

## Dependencies
As this crate uses macros to provide formatted strings it depends on the alloc crate. When using this crate
therefore a heap memory allocator has to be provided to successfully build and link. This could be a custom baremetal
allocator as provided with the corresponding crate ``ruspiro_allocator``.

## Usage
To use the crate just add the following dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-console = "0.1.1"
```

As the console crate refers to functions and structures of the ``core::alloc`` crate the final binary need to be linked
with a custom allocator. However, the ``ruspiro-console`` can bring the RusPiRo specific allocator if you activate the
feature ``with_allocator`` like so:
```
[dependencies]
ruspiro-console = { version = "0.1.1", features = ["with_allocator"] }
```

Once the console crate is available the common macros used to output strings ``print!`` and ``println`` could be used.
However, without actually setting a console output those statements will not write any data anywhere:
```
use ruspiro_console::*;

fn demo() {
    let num: u32 = 10;
    println!("This is some text with a number: {}", num);
}
```

To actually set an active output channel you need to provide a structure that implements the ``ConsoleImpl`` trait. This
for example is done in the Uart like so:
```
impl ConsoleImpl for Uart0 {
    fn putc(&self, c: char) {
        self.send_char(c);
    }

    fn puts(&self, s: &str) {
        self.send_string(s);
    }
}
```

If this trait has been implemented this structure can be used as actual console. To use it there should be the following
code written at the earliest possible point in the main crate of the binary (e.g. the kernel)
```
use ruspiro_console::*;
use ruspiro_uart::*; // as we demonstrate the usage with the Uart.

fn demo() {
    let mut uart = Uart::new(); // create a new uart struct
    if uart.initialize(250_000_000, 115_200).is_ok() { // initialize the Uart with fixed core rate and baud rate
        CONSOLE.take_for(|cons| cons.replace(uart)); // from this point CONSOLE takes ownership of Uart
        // uncommenting the following line will give compiler error as uart is moved
        // uart.send_string("I'm assigned to a console");
    }

    // if everything went fine uart should be assigned to the console for generic output
    println!("Console is ready and display's through uart");
}
```



## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)