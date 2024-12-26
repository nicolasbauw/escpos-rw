# escpos-rw: USB communication with ESC/POS (Epson) thermal printers

Escpos-rw is a "low level" fork of escpos-rs, aiming at providing just the USB connection and read/write functions.
- bug fixes
- cleaning unused and unwanted code
- adding read functions

```rust
    // VID/PID parameters
    let printer_details = PrinterProfile::usb_builder(0x04b8, 0x0202).build();
    // Creating the printer object
    let Some(printer) = Printer::new(printer_details)? else {
        return Err(escpos_rw::Error::PrinterError(
            "No printer found !".to_string(),
        ));
    };
```

## Sending raw information

The printer object has a `write_raw` method...

```rust
// Open the cash drawer
printer.write_raw([0x1B, 0x70, 0x00, 0x7E, 0x7E])?;
```

## Reading raw information

...and a `read_raw` method:

```rust
// Reads data from printer output buffer
printer.read_raw()?;
```