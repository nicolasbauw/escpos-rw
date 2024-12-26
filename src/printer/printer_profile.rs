/// Available connections with the printer
///
/// Determines the kind of connection that will be sustained with the printer. At the moment, only Usb and Terminal are implemented. Try not to use this enum directly, use the builder pattern instead (using the [usb_builder](PrinterProfile::usb_builder) or [usb_builder](PrinterProfile::terminal_builder) methods. `network_builder` soon to be available).
#[derive(Clone, Debug)]
pub struct UsbConnectionData {
    /// Vendor id for the printer
    pub vendor_id: u16,
    /// product id for the printer
    pub product_id: u16,
    /// Endpoint where the usb data is meant to be written to
    pub endpoint_w: Option<u8>,
    /// Endpoint where the usb data is meant to be read from
    pub endpoint_r: Option<u8>,
    /// Timeout for bulk write operations
    pub timeout: std::time::Duration
}

/// Details required to connect and print
///
/// In order to use the full functionality of the library, some information should be provided regarding the printer. The bare minimum information needed is the product id and the vendor id.
#[derive(Clone, Debug)]
pub struct PrinterProfile {
    /// Existing connection to the printer
    pub (crate) printer_connection_data: UsbConnectionData,
}

impl PrinterProfile {
    /// Create custom printing details
    ///
    /// Not recommended to use, as it contains a lot of arguments. See one of the builders instead (at the moment, only [usb_builder](PrinterProfile::usb_builder) and [terminal_builder](PrinterProfile::terminal_builder) available).
    pub fn new(printer_connection_data: UsbConnectionData) -> PrinterProfile {
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
}

/// Helper structure to create a [PrinterProfile](crate::PrinterProfile)
///
/// Builder pattern for the [PrinterProfile](crate::PrinterProfile) structure.
pub struct PrinterProfileBuilder {
    /// The connection to the printer
    printer_connection_data: UsbConnectionData,
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
    /// The data structure will be properly built just with the vendor id and the product id. The [Printer](crate::Printer)'s [new](crate::Printer::new) method will try to locate a bulk write endpoint, but it might fail to do so.
    pub fn new_usb(vendor_id: u16, product_id: u16) -> PrinterProfileBuilder {
        PrinterProfileBuilder {
            printer_connection_data: UsbConnectionData {
                vendor_id,
                product_id,
                endpoint_w: None,
                endpoint_r: None,
                timeout: std::time::Duration::from_secs(2)
            },
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