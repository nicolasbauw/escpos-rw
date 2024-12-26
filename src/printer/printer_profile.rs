/// Available connections with the printer
///
/// Determines the kind of connection that will be sustained with the printer. At the moment, only Usb and Terminal are implemented. Try not to use this enum directly, use the builder pattern instead (using the [usb_builder](PrinterProfile::usb_builder) or [usb_builder](PrinterProfile::terminal_builder) methods. `network_builder` soon to be available).
#[derive(Clone, Debug)]
pub enum PrinterConnectionData {
    /// Usb connection
    Usb {
        /// Vendor id for the printer
        vendor_id: u16,
        /// product id for the printer
        product_id: u16,
        /// Endpoint where the usb data is meant to be written to
        endpoint_w: Option<u8>,
        /// Endpoint where the usb data is meant to be read from
        endpoint_r: Option<u8>,
        /// Timeout for bulk write operations
        timeout: std::time::Duration
    },
    /// Network connection (not implemented yet)
    Network {
        _host: String,
        _port: u16
    },
    /// Terminal printer, used for really simple previews.
    Terminal
}

/// Details required to connect and print
///
/// In order to use the full functionality of the library, some information should be provided regarding the printer. The bare minimum information needed is the product id and the vendor id.
#[derive(Clone, Debug)]
pub struct PrinterProfile {
    /// Existing connection to the printer
    pub (crate) printer_connection_data: PrinterConnectionData,
}

impl PrinterProfile {
    /// Create custom printing details
    ///
    /// Not recommended to use, as it contains a lot of arguments. See one of the builders instead (at the moment, only [usb_builder](PrinterProfile::usb_builder) and [terminal_builder](PrinterProfile::terminal_builder) available).
    pub fn new(printer_connection_data: PrinterConnectionData) -> PrinterProfile {
        PrinterProfile {
            printer_connection_data,
        }
    }

    /// Creates a [PrinterProfileBuilder](crate::PrinterProfileBuilder) set for usb printing.
    ///
    /// Equivalent to a call to [PrinterProfileBuilder](crate::PrinterProfileBuilder)'s [new_usb](crate::PrinterProfileBuilder::new_usb) function.
    /// ```rust
    /// use escpos_rs::PrinterProfile;
    /// // Creates a minimum data structure to connect to a printer
    /// let printer_profile = PrinterProfile::usb_builder(0x0001, 0x0001).build();
    /// ```
    pub fn usb_builder(vendor_id: u16, product_id: u16) -> PrinterProfileBuilder {
        PrinterProfileBuilder::new_usb(vendor_id, product_id)
    }

    /// Creates a [PrinterProfileBuilder](crate::PrinterProfileBuilder) set for terminal printing
    ///
    /// Equivalent to a call to [PrinterProfileBuilder](crate::PrinterProfileBuilder)'s [new_terminal](crate::PrinterProfileBuilder::new_terminal) function.
    /// ```rust
    /// use escpos_rs::PrinterProfile;
    /// // Creates a minimum data structure to connect to a printer
    /// let printer_profile = PrinterProfile::terminal_builder().build();
    /// ```
    pub fn terminal_builder() -> PrinterProfileBuilder {
        PrinterProfileBuilder::new_terminal()
    }
}

/// Helper structure to create a [PrinterProfile](crate::PrinterProfile)
///
/// Builder pattern for the [PrinterProfile](crate::PrinterProfile) structure.
pub struct PrinterProfileBuilder {
    /// The connection to the printer
    printer_connection_data: PrinterConnectionData,
}

impl PrinterProfileBuilder {
    /// Creates a new [PrinterProfileBuilder](crate::PrinterProfileBuilder) set for usb printing
    ///
    /// ```rust
    /// use escpos_rs::PrinterProfileBuilder;
    /// // Creates a minimum data structure to connect to a printer
    /// let printer_profile_builder = PrinterProfileBuilder::new_usb(0x0001, 0x0001);
    /// ```
    ///
    /// The data structure will be properly built just with the vendor id and the product id. The [Printer](crate::Printer)'s [new](crate::Printer::new) method will try to locate a bulk write endpoint, but it might fail to do so. See [with_endpoint](PrinterProfileBuilder::with_endpoint) for manual setup.
    ///
    /// By default, a width of 384 dots and the `FontA` with 32 columns of width will be loaded with the profile.
    pub fn new_usb(vendor_id: u16, product_id: u16) -> PrinterProfileBuilder {
        PrinterProfileBuilder {
            printer_connection_data: PrinterConnectionData::Usb {
                vendor_id,
                product_id,
                endpoint_w: None,
                endpoint_r: None,
                timeout: std::time::Duration::from_secs(2)
            },
        }
    }

    /// Creates a new [PrinterProfileBuilder](crate::PrinterProfileBuilder) set for terminal printing
    ///
    /// ```rust
    /// use escpos_rs::PrinterProfileBuilder;
    /// // Creates a minimum (probably non-working) data structure to connect to a printer
    /// let printer_profile_builder = PrinterProfileBuilder::new_terminal();
    /// ```
    ///
    /// The printer will have a 32-char width for printing text, and a default with of 384 (but it cannot be used, as pictures can't be printed to the terminal).
    pub fn new_terminal() -> PrinterProfileBuilder {
        PrinterProfileBuilder {
            printer_connection_data: PrinterConnectionData::Terminal,
        }
    }

    /// Build the `PrinterProfile` that lies beneath the builder
    ///
    /// ```rust
    /// # use escpos_rs::PrinterProfileBuilder;
    /// let printer_profile = PrinterProfileBuilder::new_usb(0x0001, 0x0001).build();
    /// ```
    pub fn build(self) -> PrinterProfile {
        PrinterProfile {
            printer_connection_data: self.printer_connection_data,
        }
    }
}