use crate::block::Block;
use super::rotate::{Rotation2};

pub struct Z {
    rotation: Rotation2,
    data: [[u8; 3]; 3],
}

impl Z {
    pub fn first_rotation() -> Z {
        Z {
            rotation: Rotation2::R0,
            data: [
                [  0,   0,   0],
                [255, 255,   0],
                [  0, 255, 255],
            ]
        }
    }
    fn second_rotation() -> Z {
        Z {
            rotation: Rotation2::R1,
            data: [
                [  0, 255,   0],
                [255, 255,   0],
                [255,   0,   0],
            ]
        }
    }
}

impl Block for Z {
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
            Rotation2::R0 => Z::second_rotation(),
            Rotation2::R1 => Z::first_rotation(),
        })
    }
    fn rotate_left(&self) -> Box<dyn Block> {
        Box::new(match self.rotation {
            Rotation2::R0 => Z::second_rotation(),
            Rotation2::R1 => Z::first_rotation(),
        })
    }
}
