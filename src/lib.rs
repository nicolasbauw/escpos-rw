//! escpos-rw: USB communication with ESC/POS (Epson) thermal printers
//! 
//! escpos-rw is a "low level" fork of escpos-rs, aiming at providing the USB connection and read/write functions.
//! - bug fixes
//! - cleaning unused and unwanted code
//! - adding read functions

pub use printer::Printer;
pub use error::Error;

mod printer;
mod error;
