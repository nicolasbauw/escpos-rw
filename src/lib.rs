//! escpos-rw: USB communication with ESC/POS (Epson) thermal printers
//! 
//! Escpos-rw is a "low level" fork of escpos-rs, aiming at providing just the USB connection and read/write functions.
//! - bug fixes
//! - cleaning unused and unwanted code
//! - adding read functions
//! 
//! ```rust
//! // VID/PID parameters
//! let Some(printer) = Printer::new(0x04b8, 0x04b8)? else {
//!     return Err(escpos_rw::Error::PrinterError(
//!         "No printer found !".to_string(),
//!     ));
//! };
//! ```

pub use printer::Printer;
pub use error::Error;

mod printer;
mod error;
