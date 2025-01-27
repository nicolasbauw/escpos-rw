/// Errors that this crate throws.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /// Error related to rusb
    UsbError(rusb::Error),
    /// This means no bulk endpoint could be found
    NoBulkEndpoint,
    /// I/O Error
    IoError,
    /// Generic error
    PrinterError(String),
}

impl From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Error {
        Error::IoError
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::UsbError(e) => format!("USB error: {}", e),
            Error::NoBulkEndpoint => "No bulk endpoint could be found".to_string(),
            Error::IoError => format!("I/O Error"),
            Error::PrinterError(detail) => format!("Printer error: {}", detail),
        };
        write!(formatter, "{}", content)
    }
}

impl std::error::Error for Error {}
