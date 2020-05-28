use rand::seq::SliceRandom;

use crate::block::*;

pub struct BlockGenerator {
    options: [u8; 7],
    colors: [u8; 6],
    next_option: u8,
    next_color: u8,
}

pub struct BlockGeneratorResult {
    pub block: Block,
    pub block_color: u8,
    pub generator: BlockGenerator,
}

impl BlockGenerator {

    pub fn new() -> BlockGenerator {
        BlockGenerator {
            options: BlockGenerator::random_options(),
            colors: BlockGenerator::random_colors(),
            next_option: 0,
            next_color: 0,
        }
    }

    pub fn next(self) -> BlockGeneratorResult {
        BlockGeneratorResult {
            block: self.block_from_option(self.options[self.next_option as usize]),
            block_color: self.colors[self.next_color as usize],
            generator: BlockGenerator {
                options: if self.next_option as usize == self.options.len() - 1 {
                    BlockGenerator::random_options()
                } else {
                    self.options
                },
                colors: if self.next_color as usize == self.colors.len() - 1 {
                    BlockGenerator::random_colors()
                } else {
                    self.colors
                },
                next_option: (self.next_option + 1) % self.options.len() as u8,
                next_color: (self.next_color + 1) % self.colors.len() as u8,
            }
        }
    }

    fn block_from_option(&self, option: u8) -> Block {
        match option {
            0 => Block::new(BlockType::I),
            1 => Block::new(BlockType::L),
            2 => Block::new(BlockType::J),
            3 => Block::new(BlockType::S),
            4 => Block::new(BlockType::Z),
            5 => Block::new(BlockType::T),
            _ => Block::new(BlockType::Q),
        }
    }

    fn random_options() -> [u8; 7] {
        let mut options = [0, 1, 2, 3, 4, 5, 6];
        options.shuffle(&mut rand::thread_rng()); // shuffling requires mutability
        options
    }

    fn random_colors() -> [u8; 6] {
        let mut colors = [1, 2, 3, 4, 5, 6];
        colors.shuffle(&mut rand::thread_rng()); // shuffling requires mutability
        colors
    }
}
