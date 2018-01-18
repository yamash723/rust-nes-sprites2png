use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Rom {
    pub header: INesHeader,
    pub character_rom: Vec<u8>
}

impl Rom {
    pub fn new(path: &str) -> Result<Rom, RomInitializeError> {
        let rom_bytes = try!(Rom::load_to_end(path));
        let header = try!(INesHeader::new(&rom_bytes));

        // <iNES file format>
        // Header (16 bytes)
        // Trainer, if present (0 or 512 bytes)
        // PRG ROM data (16384 * x bytes)
        // CHR ROM data, if present (8192 * y bytes)
        // 
        // refer: https://wiki.nesdev.com/w/index.php/INES

        let header_size = 0x0010;         // 16 byte
        let program_unit_size = 0x4000;   // 16384 byte
        let character_unit_size = 0x2000; // 8192 byte

        let chr_rom_start = header_size + header.prg_page as usize * program_unit_size;
        let chr_rom_end = chr_rom_start + header.chr_page as usize * character_unit_size;

        let chr_tom_data = &rom_bytes[chr_rom_start..chr_rom_end];

        Ok(Rom {
            header: header,
            character_rom: chr_tom_data.to_vec()
        })
    }

    fn load_to_end(path: &str) -> Result<Vec<u8>, RomInitializeError> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}

#[derive(Debug)]
pub enum RomInitializeError {
    IoError(io::Error),
    /// Rom haven't a magic number
    FormatError
}

impl From<io::Error> for RomInitializeError {
    fn from(err: io::Error) -> Self {
        RomInitializeError::IoError(err)
    }
}

#[derive(Debug, PartialEq)]
pub struct INesHeader {
    /// ASCII letters 'NES' followed by 0x1A(EOF)
    magic_numbers: [u8; 4],
    /// Number of pages for The program rom
    prg_page: u8,
    /// Number of pages for The character rom
    chr_page: u8
}

impl INesHeader {
    fn new(rom_bytes: &Vec<u8>) -> Result<Self, RomInitializeError> {        
        // <iNES file format header>
        // 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
        // 4: Size of PRG ROM in 16 KB units
        // 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
        // 
        // refer: https://wiki.nesdev.com/w/index.php/INES

        let header = INesHeader {
            magic_numbers: *array_ref!(&rom_bytes, 0, 4),
            prg_page: rom_bytes[4],
            chr_page: rom_bytes[5]
        };

        if header.magic_numbers != *"NES\x1A".as_bytes() {
            return Err(RomInitializeError::FormatError)
        };
        
        Ok(header)
    }
}

#[cfg(test)]
mod ines_header_test {
    use super::*;

    #[test]
    fn initialize_ok() {
        // "N" "E" "S" "\x1A" "5" "3"
        let rom_bytes = [78, 69, 83, 26, 53, 51];
        assert_eq!(rom_bytes, *"NES\x1A53".as_bytes());

        let ines_header = INesHeader::new(&rom_bytes.to_vec()).unwrap();
        assert_eq!(ines_header, INesHeader {
            magic_numbers: [
                rom_bytes[0],
                rom_bytes[1],
                rom_bytes[2],
                rom_bytes[3],
            ],
            prg_page: rom_bytes[4],
            chr_page: rom_bytes[5]
        });
    }

    #[test]
    fn initialize_ng_format_error() {
        // "N" "O" "S" "\x1A" "5" "3"
        let rom_bytes = [78, 79, 83, 26, 53, 51];
        assert_eq!(rom_bytes, *"NOS\x1A53".as_bytes());

        let ines_header = INesHeader::new(&rom_bytes.to_vec());
        assert!(
            match ines_header {
                Err(RomInitializeError::FormatError) => true,
                _ => false
            }
        );
    }
}