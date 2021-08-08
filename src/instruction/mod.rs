pub use self::instruction::{Instruction};
pub use self::print_data::{PrintData, PrintDataBuilder};
pub use self::justification::{Justification};
pub use self::escpos_image::EscposImage;

mod instruction;
mod print_data;
mod justification;
mod escpos_image;

/*
let img = match image::load_from_memory(&content) {
    Ok(i) => i,
    Err(e) => return Err(Error::ImageError(e))
};
*/