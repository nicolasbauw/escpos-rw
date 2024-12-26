pub use self::printer_profile::{PrinterProfile, PrinterConnectionData, PrinterProfileBuilder};
use std::{thread, time::Duration};

mod printer_profile;

use crate::Error;

use rusb::{UsbContext, Context, DeviceHandle, TransferType, Direction};

const OP_DELAY: u64 = 10;

/// Keeps the actual living connection to the device
enum PrinterConnection {
    Usb {
        /// Bulk write endpoint
        endpoint: u8,
        /// Bulk read endpoint
        endpoint_r: u8,
        /// Device handle
        dh: DeviceHandle<Context>,
        /// Time to wait before giving up writing to the bulk endpoint
        timeout: std::time::Duration
    },
    Terminal
}

/// Main escpos-rs structure
///
/// The printer represents the thermal printer connected to the computer.
/// ```rust,no_run
/// use escpos_rw::{Printer, PrinterModel};
///
/// let printer = match Printer::new(PrinterModel::TMT20.usb_profile()) {
///     Ok(maybe_printer) => match maybe_printer {
///         Some(printer) => printer,
///         None => panic!("No printer was found :(")
///     },
///     Err(e) => panic!("Error: {}", e)
/// };
/// // Now we have a printer
/// ```
pub struct Printer {
    /// Actual connection to the printer
    printer_connection: PrinterConnection,
}

impl Printer {
    /// Creates a new printer
    /// 
    /// Creates the printer with the given details, from the printer details provided, and in the given USB context.
    pub fn new(printer_profile: PrinterProfile) -> Result<Option<Printer>, Error> {
        // Quick check for the profile containing at least one font
        match printer_profile.printer_connection_data {
            PrinterConnectionData::Usb{vendor_id, product_id, endpoint_w, endpoint_r, timeout} => {
                let context = Context::new().map_err(Error::RusbError)?;
        
                let devices = context.devices().map_err(Error::RusbError)?;
                for device in devices.iter() {
                    let s = device.device_descriptor().map_err(Error::RusbError)?;
                    if s.vendor_id() == vendor_id && s.product_id() == product_id {
                        // Before opening the device, we must find the bulk endpoint
                        let config_descriptor = device.active_config_descriptor().map_err(Error::RusbError)?;
                        let actual_endpoint = if let Some(endpoint_w) = endpoint_w {
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

                        let actual_endpoint_r = if let Some(endpoint_r) = endpoint_r {
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
                            Ok(mut dh) => {
                                if let Ok(active) = dh.kernel_driver_active(0) {
                                    if active {
                                        // The kernel is active, we have to detach it
                                        match dh.detach_kernel_driver(0) {
                                            Ok(_) => (),
                                            Err(e) => return Err(Error::RusbError(e))
                                        };
                                    }
                                } else {
                                    println!("Could not find out if kernel driver is active, might encounter a problem soon.");
                                };
                                // Now we claim the interface
                                match dh.claim_interface(0) {
                                    Ok(_) => (),
                                    Err(e) => return Err(Error::RusbError(e))
                                }
                                return Ok(Some(Printer {
                                    printer_connection: PrinterConnection::Usb {
                                        endpoint: actual_endpoint,
                                        endpoint_r: actual_endpoint_r,
                                        dh,
                                        timeout
                                    },
                                }));
                            },
                            Err(e) => return Err(Error::RusbError(e))
                        };
                    }
                }
                // No printer was found with such vid and pid
                Ok(None)
            },
            PrinterConnectionData::Network{..} => panic!("Unsupported!"),
            PrinterConnectionData::Terminal => Ok(Some(Printer{
                printer_connection: PrinterConnection::Terminal,
            }))
        }
    }

    /// Sends raw information to the printer
    ///
    /// As simple as it sounds
    /// ```rust,no_run
    /// use escpos_rw::{Printer,PrinterProfile};
    /// let printer_profile = PrinterProfile::usb_builder(0x0001, 0x0001).build();
    /// let printer = Printer::new(printer_profile).unwrap().unwrap();
    /// printer.write_raw(&[0x01, 0x02])?;
    /// # Ok::<(), escpos_rw::Error>(())
    /// ```
    pub fn write_raw<A: AsRef<[u8]>>(&self, bytes: A) -> Result<(), Error> {
        match &self.printer_connection {
            PrinterConnection::Usb{endpoint, endpoint_r: _, dh, timeout} => {
                dh.write_bulk(
                    *endpoint,
                    bytes.as_ref(),
                    *timeout
                ).map_err(Error::RusbError)?;
                thread::sleep(Duration::from_millis(OP_DELAY));
                Ok(())
            },
            _other => panic!("Unimplemented")
        }
    }

    /// Reads raw information from the printer
    ///
    /// ```rust,no_run
    /// use escpos_rw::{Printer,PrinterProfile};
    /// let printer_profile = PrinterProfile::usb_builder(0x0001, 0x0001).build();
    /// let printer = Printer::new(printer_profile).unwrap().unwrap();
    /// let buffer = printer.read_raw()?;
    /// # Ok::<(), escpos_rw::Error>(())
    /// ```
    pub fn read_raw(&self) -> Result<[u8; 20], Error> {
        match &self.printer_connection {
            PrinterConnection::Usb{endpoint: _, endpoint_r,dh, timeout} => {
                let mut buffer: [u8; 20] = [0; 20];
                dh.read_bulk(
                    *endpoint_r,
                    &mut buffer,
                    *timeout
                ).map_err(Error::RusbError)?;
                Ok(buffer)
            },
            _other => panic!("Unimplemented")
        }
    }
}