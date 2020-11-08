/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: MIT / Appache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-console/||VERSION||")]
#![cfg_attr(not(any(test, doctest)), no_std)]

//! # Console abstraction
//!
//! This crate provides a console abstraction to enable string output to a configurable output channel.
//! It also provides the convinient macros (``print!`` and ``println!``) to output text that are usually not
//! available in ``[no_std]`` environments. However this crate also provide macros to indicate the severity of the
//! message that shall be printed. Those are ``info!``, ``warn!`` and ``error!``.
//!
//! # Dependencies
//! This crate uses macros to provide formatted strings. This formatting requires a memory allocator to
//! be present (as part of the ``alloc`` crate). So when using this crate provide an allocator such as
//! ``ruspiro_allocator``.
//!
//! # Example
//! To actually set an active output channel you need to provide a structure that implements the ``core::fmt::Write`` 
//! trait.
//!
//! If this trait has been implemented this structure can be used as actual console. To use it there should be the 
//! following code written at the earliest possible point in the main crate of the binary (e.g. the kernel)
//!
//! ```ignore
//! use ruspiro_console::*;
//! use ruspiro_uart::*; // as we demonstrate with the Uart as output device/channel.
//!
//! fn main() {
//!     let mut uart = Uart1::new(); // create a new uart struct
//!     if uart.initialize(250_000_000, 115_200).is_ok() { // initialize the Uart with fixed core rate and baud rate
//!         CONSOLE.take_for(|cons| cons.replace(uart)); // from this point CONSOLE takes ownership of uart
//!     }
//!
//!     // if everything went fine uart should be assigned to the console for generic output
//!     println!("Console is ready and display's through uart");
//! }
//! ```

pub extern crate alloc;

#[macro_use]
pub mod macros;
pub use macros::*;

use alloc::boxed::Box;
use ruspiro_singleton::Singleton;
use core::fmt;

/// The Console singleton used by print! and println! macros
pub static CONSOLE: Singleton<Console> = Singleton::new(Console {
    current: None,
    default: DefaultConsole {},
});

#[doc(hidden)]
/// The base printing function hidden behind the print! and println! macro. This function forwards all calls to the
/// generic console which writes the string to the assigned output channel.
pub fn _print(args: fmt::Arguments) {
    // pass the string to the actual configured console to be printed
    CONSOLE.take_for(|console| {
        if let Some(ref mut writer) = console.current {
            writer.write_fmt(args).expect("writing to console should never fail");
        }
    });
}

/// The representation of the abstract console
#[allow(dead_code)]
pub struct Console {
    current: Option<Box<dyn fmt::Write>>,
    default: DefaultConsole,
}

impl Console {
    /// Replacing the current active console. Once the new has been set the [drop] function of the previous one is
    /// called. The Console takes ownership of the active once. Access to the active console outside the abstraction
    /// is not possible and should not be.
    pub fn replace<T: fmt::Write + 'static>(&mut self, console: T) {
        self.current.replace(Box::from(console));
    }
}

/// The default console is a kind of fall back that prints nothing...
struct DefaultConsole;

impl fmt::Write for DefaultConsole {
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        Ok(())
    }
}
