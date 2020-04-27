use rand::seq::SliceRandom;

use crate::block::Block;
use crate::blocks::*;

pub struct BlockGenerator {
    options: [u8; 7],
    colors: [u8; 6],
    next_option: u8,
    next_color: u8,
}

pub struct BlockGeneratorResult {
    pub block: Box<dyn Block>,
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

    fn block_from_option(&self, option: u8) -> Box<dyn Block> {
        match option {
            0 => Box::new(i::I::first_rotation()),
            1 => Box::new(l::L::first_rotation()),
            2 => Box::new(j::J::first_rotation()),
            3 => Box::new(s::S::first_rotation()),
            4 => Box::new(z::Z::first_rotation()),
            5 => Box::new(t::T::first_rotation()),
            _ => Box::new(q::Q::first_rotation()),
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
