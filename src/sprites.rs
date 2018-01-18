pub struct Sprites {
    sprites: Vec<Sprite>
}

impl Sprites {
    pub fn new(character_rom: &Vec<u8>) -> Self {
        let byte_sprites = Sprites::split_by_sprite_size(character_rom);
        let sprites = byte_sprites.iter()
                                  .map(|b| Sprite::from_bytes(b))
                                  .collect();

        Sprites { sprites: sprites }
    }
    
    pub fn get<'a>(&'a self, index: usize) -> &'a Sprite {
        &self.sprites[index]
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    fn split_by_sprite_size(character_rom: &Vec<u8>) -> Vec<[u8; 16]> {
        let sprite_size = 16usize;

        let sprites_number = character_rom.len() / sprite_size;
        let mut splitted_vec = Vec::new();
        
        for num in 0..sprites_number {
            let start = sprite_size * num as usize;
            splitted_vec.push(*array_ref!(&character_rom, start, 16));
        }

        splitted_vec
    }
}

#[derive(Debug, PartialEq)]
pub struct Sprite {
    pub color_numbers : Vec<Vec<u8>>
}

impl Sprite {
    fn from_bytes(bytes: &[u8; 16]) -> Sprite {
        let channel_1 = &bytes[0..8];
        let channel_2 = &bytes[8..16];

        Sprite {
            color_numbers: Sprite::overlap_two_channel(channel_1, channel_2)
        }
    }

    fn overlap_two_channel(channel_1: &[u8], channel_2: &[u8]) -> Vec<Vec<u8>> {
        let mut overlapped_channel = vec![vec![0u8; 8]; 8];

        for y in 0..8 {
            for x in 0..8 {
                let shift = 7 - x;
                let get_target_bit = |byte: u8| -> u8 { (byte & (0x80 >> x)) >> shift };
                let p1 = get_target_bit(channel_1[y]);
                let p2 = get_target_bit(channel_2[y]);
                overlapped_channel[y][x] = p1 + (p2 * 2);
            }
        }

        overlapped_channel
    }
}

#[cfg(test)]
mod sprites_test {
    use super::*;

    #[test]
    fn len() {
        let character_rom = &vec![0u8; 32];
        let sprites = Sprites::new(character_rom);

        assert_eq!(2, sprites.len());
    }

    #[test]
    fn get() {
        let character_rom = &vec![0u8; 32];
        let sprites = Sprites::new(character_rom);
        let sprite = Sprite::from_bytes(&[0u8; 16]);

        assert_eq!(sprites.get(0), &sprite);
        assert_eq!(sprites.get(1), &sprite);
    }

    #[test]
    fn split_by_sprite_size() {
        let character_rom = &vec![
            0,0,0,0,0,0,0,0,  0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,1,1,  1,1,1,1,1,1,1,1,
            2,2,2,2,2,2,2,2,  2,2,2,2,2,2,2,2,
            3,3,3,3,3,3,3,3,  3,3,3,3,3,3,3,3,
        ];

        let splitted_rom = Sprites::split_by_sprite_size(character_rom);

        assert_eq!(splitted_rom, vec![
            [0u8; 16],
            [1u8; 16],
            [2u8; 16],
            [3u8; 16],
        ]);
    }
}

#[cfg(test)]
mod sprite_test {
    use super::*;

    #[test]
    fn from_bytes() {
        let byte_sprite = &[
            0b11111000u8,
            0b11111000u8,
            0b11111000u8,
            0b11111000u8,
            0b11111000u8,
            0b00000000u8,
            0b00000000u8,
            0b00000000u8,

            0b00000000u8,
            0b00000000u8,
            0b00000000u8,
            0b00011111u8,
            0b00011111u8,
            0b00011111u8,
            0b00011111u8,
            0b00011111u8,
        ];

        let decode_sprite = Sprite::from_bytes(byte_sprite);
        
        assert_eq!(decode_sprite, Sprite {
            color_numbers: vec![
                vec![1,1,1,1,1,0,0,0],
                vec![1,1,1,1,1,0,0,0],
                vec![1,1,1,1,1,0,0,0],
                vec![1,1,1,3,3,2,2,2],
                vec![1,1,1,3,3,2,2,2],
                vec![0,0,0,2,2,2,2,2],
                vec![0,0,0,2,2,2,2,2],
                vec![0,0,0,2,2,2,2,2],
            ]});
    }

    #[test]
    fn overlap_two_channel() {
        let channel_1 = &[
            0b11111000u8,
            0b11111000u8,
            0b11111000u8,
            0b11111000u8,
            0b11111000u8,
            0b00000000u8,
            0b00000000u8,
            0b00000000u8,
        ];

        let channel_2 = &[
            0b00000000u8,
            0b00000000u8,
            0b00000000u8,
            0b00011111u8,
            0b00011111u8,
            0b00011111u8,
            0b00011111u8,
            0b00011111u8,
        ];

        let overlapped_channel = Sprite::overlap_two_channel(channel_1, channel_2);

        assert_eq!(overlapped_channel, vec![
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,1,1,0,0,0],
            vec![1,1,1,3,3,2,2,2],
            vec![1,1,1,3,3,2,2,2],
            vec![0,0,0,2,2,2,2,2],
            vec![0,0,0,2,2,2,2,2],
            vec![0,0,0,2,2,2,2,2],
        ]);
    }
}