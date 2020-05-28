use std::iter::Iterator;

use crate::block::Block;

pub struct Board {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn compress(&self) -> Board {
        let filtered_rows: Vec<u8> = self.data
            .chunks(self.width)
            .filter(|row| row.iter().any(|cell| *cell == 0))
            .flatten()
            .map(|x| *x)
            .collect();

        let full_data = (0..(self.width * self.height - filtered_rows.len()))
            .map(|_| 0u8)
            .chain(filtered_rows.iter().map(|x| *x))
            .collect();

        Board {
            data: full_data,
            ..*self
        }
    }

    pub fn with_block(&self, block: &Block, position: BlockPosition, color: u8) -> Board {
        let block_at_abs = |x, y| {
            let block_x = x as i32 - position.x;
            let block_y = y as i32 - position.y;

            if block_x >= 0
                && block_x < 4
                && block_y >= 0
                && block_y < 4 {
                block.at(block_x as usize, block_y as usize)
            } else {
                0
            }
        };
        Board {
            data: self.data.iter().enumerate().map(|(i, v)| {
                if *v != 0 {
                    *v
                } else {
                    let x = i % self.width;
                    let y = i / self.width;
                    block_at_abs(x, y) & color
                }
            }).collect(),
            ..*self
        }
    }

    pub fn block_collides(&self, block: &Block, position: BlockPosition) -> bool {
        // not all blocks can be moved without overlap
        !(0..4).all(|y| {
            (0..4).all(|x| {
                let board_x = x as i32 + position.x;
                let board_y = y as i32 + position.y;
                let block_value = block.at(x, y);

                let board_value = if board_x < 0 {
                    255
                } else if board_x >= self.width as i32 {
                    255
                } else if board_y >= self.height as i32 {
                    255
                } else if board_y < 0 {
                    0
                } else {
                    self.data[(self.width as i32 * board_y + board_x) as usize]
                };

                board_value == 0 || block_value == 0
            })
        })
    }

    pub fn render(&self) -> String {
        let bar = String::from("═").repeat(self.width * 2);

        let middle = self.data.chunks(self.width).map(|chunk| {
            let row: String = chunk.iter().map(|v| match *v {
                0 => "  ",
                1 => "\x1b[31m██\x1b[0m",
                2 => "\x1b[32m██\x1b[0m",
                3 => "\x1b[33m██\x1b[0m",
                4 => "\x1b[34m██\x1b[0m",
                5 => "\x1b[35m██\x1b[0m",
                6 => "\x1b[36m██\x1b[0m",
                _ => "\x1b[37m██\x1b[0m",
            }).collect();
            String::from("║") + &row + "║\n"
        }).collect::<String>();

        String::from("") +
            "╔" + &bar + "╗\n" +
            &middle +
            "╚" + &bar + "╝\n"
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
