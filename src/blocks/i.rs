use crate::block::Block;
use super::rotate::{Rotation2};

pub struct I {
    rotation: Rotation2,
    data: [[u8; 4]; 4],
}

impl I {
    pub fn first_rotation() -> I {
        I {
            rotation: Rotation2::R0,
            data: [
                [   0,   0, 255,   0],
                [   0,   0, 255,   0],
                [   0,   0, 255,   0],
                [   0,   0, 255,   0],
            ]
        }
    }
    fn second_rotation() -> I {
        I {
            rotation: Rotation2::R1,
            data: [
                [   0,   0,   0,   0],
                [   0,   0,   0,   0],
                [ 255, 255, 255, 255],
                [   0,   0,   0,   0],
            ]
        }
    }
}

impl Block for I {
    fn width(&self) -> usize {
        4
    }
    fn height(&self) -> usize {
        4
    }
    fn at(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }
    fn rotate_right(&self) -> Box<dyn Block> {
        Box::new(match self.rotation {
            Rotation2::R0 => I::second_rotation(),
            Rotation2::R1 => I::first_rotation(),
        })
    }
    fn rotate_left(&self) -> Box<dyn Block> {
        Box::new(match &self.rotation {
            Rotation2::R0 => I::second_rotation(),
            Rotation2::R1 => I::first_rotation(),
        })
    }
}
