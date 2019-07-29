/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Appache License 2.0
 **********************************************************************************************************************/

//! # Convinient output macros to print formatted strings to the configured channel of the console
//! 

/// Provide the print!() and println!() macro's as used in the libstd crate which is not available here
/// as we do need formatting on the parameter and formatting requires memory allocation the
/// use of this functions is only possible if a global allocator is implemented.<br>
/// See the [rubots-memory] crate for the a possible implementation.
#[macro_export]
macro_rules! print {
    //$crate::macros::alloc::
    ($($arg:tt)*) => ($crate::print($crate::alloc::format!($($arg)*).as_str()));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ({
        $crate::print!("{}\r\n", $crate::alloc::format!($($arg)*));
    })
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::print!("I: {} - {}\r\n", module_path!(), $crate::alloc::format!($($arg)*));
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::print!("W: {} - {}\r\n", module_path!(), $crate::alloc::format!($($arg)*));
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::print!("E: {} - {}\r\n", module_path!(), $crate::alloc::format!($($arg)*));
    })
}