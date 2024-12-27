use rusb::{UsbContext, Context, DeviceHandle, TransferType, Direction};
use std::{thread, time::Duration};
use crate::Error;

const OP_DELAY: u64 = 10;

struct PrinterConnection {
    /// Bulk write endpoint
    endpoint: u8,
    /// Bulk read endpoint
    endpoint_r: u8,
    /// Device handle
    dh: DeviceHandle<Context>,
    /// Time to wait before giving up writing to the bulk endpoint
    timeout: std::time::Duration
}

struct UsbConnectionData {
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

/// The printer object represents the thermal printer.
pub struct Printer {
    /// Actual connection to the printer
    printer_connection: PrinterConnection,
}

impl Printer {
    /// Creates the printer with the given VID/PID
    /// ```rust,no_run
    /// use escpos_rw::{Error, Printer};
    /// # fn main() -> Result<(), Error> {
    /// let Some(printer) = Printer::new(0x04b8, 0x0202)? else {
    ///     return Err(escpos_rw::Error::PrinterError(
    ///         "No printer found !".to_string(),
    ///     ));
    /// };
    /// # Ok(())}
    /// ```
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Option<Printer>, Error> {
        let printer_connection_data =  UsbConnectionData {
            vendor_id,
            product_id,
            endpoint_w: None,
            endpoint_r: None,
            timeout: std::time::Duration::from_secs(2)
        };

        // Quick check for the profile containing at least one font
                let context = Context::new().map_err(Error::UsbError)?;
        
                let devices = context.devices().map_err(Error::UsbError)?;
                for device in devices.iter() {
                    let s = device.device_descriptor().map_err(Error::UsbError)?;
                    if s.vendor_id() == printer_connection_data.vendor_id && s.product_id() == printer_connection_data.product_id {
                        // Before opening the device, we must find the bulk endpoint
                        let config_descriptor = device.active_config_descriptor().map_err(Error::UsbError)?;
                        let actual_endpoint = if let Some(endpoint_w) = printer_connection_data.endpoint_w {
                            endpoint_w
                        } else {
                            let mut detected_endpoint: Option<u8> = None;
                            // Horrible to have 3 nested for, but so be it
                            for interface in config_descriptor.interfaces() {
                                for descriptor in interface.descriptors() {
                                    for endpoint in descriptor.endpoint_descriptors() {
                                        if let (TransferType::Bulk, Direction::Out) = (endpoint.transfer_type(), endpoint.direction()) {
                                            detected_endpoint = Some(endpoint.address());   
                                        }
                                    }
                                }
                            }
            
                            if let Some(detected_endpoint) = detected_endpoint {
                                detected_endpoint
                            } else {
                                return Err(Error::NoBulkEndpoint);
                            }

                        };

                        let actual_endpoint_r = if let Some(endpoint_r) = printer_connection_data.endpoint_r {
                            endpoint_r
                        } else {
                            let mut detected_endpoint_r: Option<u8> = None;
                            // Horrible to have 3 nested for, but so be it
                            for interface in config_descriptor.interfaces() {
                                for descriptor in interface.descriptors() {
                                    for endpoint in descriptor.endpoint_descriptors() {
                                        if let (TransferType::Bulk, Direction::In) = (endpoint.transfer_type(), endpoint.direction()) {
                                            detected_endpoint_r = Some(endpoint.address());
                                        }
                                    }
                                }
                            }
            
                            if let Some(detected_endpoint_r) = detected_endpoint_r {
                                detected_endpoint_r
                            } else {
                                return Err(Error::NoBulkEndpoint);
                            }

                        };
        
                        // Now we continue opening the device
        
                        match device.open() {
                            Ok(dh) => {
                                if let Ok(active) = dh.kernel_driver_active(0) {
                                    if active {
                                        // The kernel is active, we have to detach it
                                        match dh.detach_kernel_driver(0) {
                                            Ok(_) => (),
                                            Err(e) => return Err(Error::UsbError(e))
                                        };
                                    }
                                } else {
                                    println!("Could not find out if kernel driver is active, might encounter a problem soon.");
                                };
                                // Now we claim the interface
                                match dh.claim_interface(0) {
                                    Ok(_) => (),
                                    Err(e) => return Err(Error::UsbError(e))
                                }
                                let timeout = printer_connection_data.timeout;
                                return Ok(Some(Printer {
                                    printer_connection: PrinterConnection {
                                        endpoint: actual_endpoint,
                                        endpoint_r: actual_endpoint_r,
                                        dh,
                                        timeout
                                    },
                                }));
                            },
                            Err(e) => return Err(Error::UsbError(e))
                        };
                    }
                }
                // No printer was found with such vid and pid
                Ok(None)
    }

    /// Sends bytes to the printer
    /// ```rust,no_run
    /// # use escpos_rw::{Error, Printer};
    /// # fn main() -> Result<(), Error> {
    /// # use escpos_rw::{Error, Printer};
    /// # let Some(printer) = Printer::new(0x04b8, 0x0202)? else {
    /// # return Err(escpos_rw::Error::PrinterError(
    /// #     "No printer found !".to_string(),
    /// # ));
    /// # };
    /// // Open the cash drawer
    /// printer.write_raw([0x1B, 0x70, 0x00, 0x7E, 0x7E])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_raw<A: AsRef<[u8]>>(&self, bytes: A) -> Result<(), Error> {
        match &self.printer_connection {
            PrinterConnection {endpoint, endpoint_r: _, dh, timeout} => {
                dh.write_bulk(
                    *endpoint,
                    bytes.as_ref(),
                    *timeout
                ).map_err(Error::UsbError)?;
                thread::sleep(Duration::from_millis(OP_DELAY));
                Ok(())
            }
        }
    }

    /// Reads bytes from the printer
    /// ```rust,no_run
    /// # use escpos_rw::{Error, Printer};
    /// # fn main() -> Result<(), Error> {
    /// # use escpos_rw::{Error, Printer};
    /// # let Some(printer) = Printer::new(0x04b8, 0x0202)? else {
    /// # return Err(escpos_rw::Error::PrinterError(
    /// #     "No printer found !".to_string(),
    /// # ));
    /// # };
    /// // Reads data from printer output buffer
    /// printer.read_raw()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_raw(&self) -> Result<[u8; 20], Error> {
        match &self.printer_connection {
            PrinterConnection{endpoint: _, endpoint_r,dh, timeout} => {
                let mut buffer: [u8; 20] = [0; 20];
                dh.read_bulk(
                    *endpoint_r,
                    &mut buffer,
                    *timeout
                ).map_err(Error::UsbError)?;
                Ok(buffer)
            },
        }
    }
}