# Simple Console abstraction RusPiRo crate

## Usage
To use the crate just add the following dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-console = { git = "https://github.com/RusPiRo/ruspiro-console", tag = "v0.0.1" }
```

As the console crate refers to functions and structures of the ``core::alloc`` crate the final binary need to be linked
with a custom allocator. However, the ``ruspiro-console`` can bring the RusPiRo specific allocator if you activate the
feature ``with_allocator`` like so:
```
[dependencies]
ruspiro-console = { git = "https://github.com/RusPiRo/ruspiro-console", tag = "v0.0.1", features = ["with_allocator"] }
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
for excample is done in the Uart like so:
```
impl ConsoleImpl for Uart0 {
    fn puts(&self, s: &str) {
        self.send_string(s);
    }
}
```

If this trait has been implemented this structure can be used as actual console. To use it there should be the following
code written at the earliest possible point in the main crate of the binary (e.g. the kernel)
```
use ruspiro_console::*;
use ruspiro_uart::*; // as we demonstrate with the Uart.

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
This crate is licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)