use image;
use std::fs::File;

use sprites::*;

pub struct SpriteImageWriter {
    /// Count of sprite per row
    count_per_row: u32,
    sprites: Sprites
}

impl SpriteImageWriter {
    pub fn new(sprites: Sprites, count_per_row: u32) -> Self {
        SpriteImageWriter {
            sprites: sprites,
            count_per_row: count_per_row
        }
    }

    pub fn save(self, path: &str) {
        let width = 8 * self.count_per_row;
        let height = 8 * self.row_count();
        let mut image_buffer  = image::ImageBuffer::new(width, height);

        for i in 0..self.sprites.len() {
            let sprite = self.sprites.get(i);
            let color_numbers = &sprite.color_numbers;
            for y in 0..8 {
                for x in 0..8 {
                    let pos_x = x + (i as u32 % self.count_per_row * 8);
                    let pos_y = y + (i as u32 / self.count_per_row * 8);
                    let color_number = color_numbers[y as usize][x as usize];
                    let pixel = SpriteImageWriter::color_number_to_pixel(color_number);

                    image_buffer.put_pixel(pos_x, pos_y, pixel);
                }   
            }
        }
        
        let ref mut output_file = File::create(path).unwrap();
        image::ImageLuma8(image_buffer).save(output_file, image::PNG).unwrap();
    }

    fn row_count(&self) -> u32 {
        (self.sprites.len() as f32 / self.count_per_row as f32).ceil() as u32
    }

    fn color_number_to_pixel(color_number: u8) -> image::Luma<u8> {        
        match color_number {
            0 => image::Luma([0u8]),
            1 => image::Luma([117u8]),
            2 => image::Luma([188u8]),
            3 => image::Luma([255u8]),
            _ => image::Luma([0u8]),
        }
    }
}

#[cfg(test)]
mod sprite_image_writer_test {
    use super::*;

    #[test]
    fn row_count_by_divisible() {
        let character_rom = &vec!(0u8; 16 * 10);
        let sprites = Sprites::new(character_rom);

        let writer = SpriteImageWriter::new(sprites, 2);
        assert_eq!(writer.row_count(), 5);
    }

    #[test]
    fn row_count_by_indivisible() {
        let character_rom = &vec!(0u8; 16 * 10);
        let sprites = Sprites::new(character_rom);

        let writer = SpriteImageWriter::new(sprites, 3);
        let row_count = writer.row_count();
        assert!(row_count != 3);
        assert_eq!(row_count, 4);
    }

    #[test]
    fn color_number_to_pixel() {
        let white =      image::Luma([255u8]);
        let right_gray = image::Luma([188u8]);
        let gray =       image::Luma([117u8]);
        let black =      image::Luma([0u8]);

        assert_eq!(SpriteImageWriter::color_number_to_pixel(3), white);
        assert_eq!(SpriteImageWriter::color_number_to_pixel(2), right_gray);
        assert_eq!(SpriteImageWriter::color_number_to_pixel(1), gray);
        assert_eq!(SpriteImageWriter::color_number_to_pixel(0), black);
    }
}