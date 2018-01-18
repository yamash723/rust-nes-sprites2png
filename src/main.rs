#[macro_use]
extern crate arrayref;
extern crate image;

use std::env;

mod rom;
mod sprites;
mod writer;

use rom::*;
use sprites::*;
use writer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("require input a rom & output file path!");
    }

    let rom_file_path = &args[1];
    let output_file_paht = &args[2];

    let rom = match Rom::new(rom_file_path) {
        Ok(rom) => rom,
        Err(RomInitializeError::FormatError) =>
            panic!("{} is not a NES file", output_file_paht),
        Err(RomInitializeError::IoError(err)) =>
            panic!("can't load a rom file. {}", err),
    };

    let init_count_per_row = 50;
    let sprites = Sprites::new(&rom.character_rom);
    let writer = SpriteImageWriter::new(sprites, init_count_per_row);
    writer.save(output_file_paht);
}