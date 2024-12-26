//! escpos-rw: USB communication with ESC/POS (Epson) thermal printers
//! 
//! Escpos-rw is a "low level" fork of escpos-rs, aiming at providing just the USB connection and read/write functions.
//! - bug fixes
//! - cleaning unused and unwanted code
//! - adding read functions
//! 
//! ```rust
//! // VID/PID parameters
//! let printer_details = PrinterProfile::usb_builder(0x04b8, 0x0202).build();
//! // Creating the printer object
//! let Some(printer) = Printer::new(printer_details)? else {
//!     return Err(escpos_rw::Error::PrinterError(
//!         "No printer found !".to_string(),
//!     ));
//! };
//! ```

pub use printer::{Printer, PrinterProfile, PrinterProfileBuilder, PrinterConnectionData};
pub use error::Error;

mod printer;
mod error;
