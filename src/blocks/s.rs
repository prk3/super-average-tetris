use crate::block::Block;
use super::rotate::{Rotation2};

pub struct S {
    rotation: Rotation2,
    data: [[u8; 3]; 3],
}

impl S {
    pub fn first_rotation() -> S {
        S {
            rotation: Rotation2::R0,
            data: [
                [  0,   0,   0],
                [  0, 255, 255],
                [255, 255,   0],
            ]
        }
    }
    fn second_rotation() -> S {
        S {
            rotation: Rotation2::R1,
            data: [
                [  0, 255,   0],
                [  0, 255, 255],
                [  0,   0, 255],
            ]
        }
    }
}

impl Block for S {
    fn width(&self) -> usize {
        3
    }
    fn height(&self) -> usize {
        3
    }
    fn at(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }
    fn rotate_right(&self) -> Box<dyn Block> {
        Box::new(match self.rotation {
            Rotation2::R0 => S::second_rotation(),
            Rotation2::R1 => S::first_rotation(),
        })
    }
    fn rotate_left(&self) -> Box<dyn Block> {
        Box::new(match self.rotation {
            Rotation2::R0 => S::second_rotation(),
            Rotation2::R1 => S::first_rotation(),
        })
    }
}
