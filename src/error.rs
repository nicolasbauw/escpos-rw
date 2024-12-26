/// Errors that this crate throws.
#[derive(Debug)]
pub enum Error {
    /// Error related to rusb
    UsbError(rusb::Error),
    /// This means no bulk endpoint could be found
    NoBulkEndpoint,
    /// Generic error
    PrinterError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let content = match self {
            Error::UsbError(e) => format!("rusb error: {}", e),
            Error::NoBulkEndpoint => "No bulk endpoint could be found".to_string(),
            Error::PrinterError(detail) => format!("An error occured while printing, {}", detail)
        };
        write!(formatter, "{}", content)
    }
}

impl std::error::Error for Error{}