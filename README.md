# escpos-rw: USB communication with ESC/POS (Epson) thermal printers

Escpos-rw is a "low level" fork of escpos-rs, aiming at providing just the USB connection and read/write functions.
- bug fixes
- cleaning unused and unwanted code
- adding read functions

You can find ESC/POS Command Reference [here](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/).

## Creating the printer object

```rust
// Creating the printer object
    let Some(printer) = Printer::new(0x04b8, 0x0202)? else {
        return Err(escpos_rw::Error::PrinterError(
            "No printer found !".to_string(),
        ));
    };
```

## Sending data

The printer object has a `write_raw` method...

```rust
// Open the cash drawer
printer.write_raw([0x1B, 0x70, 0x00, 0x7E, 0x7E])?;
```

## Reading data

...and a `read_raw` method:

```rust
// Reads data from printer output buffer
printer.read_raw()?;
```