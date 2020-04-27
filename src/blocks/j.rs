use crate::block::Block;
use super::rotate::{rotate_square_3_left, rotate_square_3_right};

pub struct J {
    data: [[u8; 3]; 3],
}

impl J {
    pub fn first_rotation() -> J {
        J {
            data: [
                [   0,   0,   0],
                [ 255, 255, 255],
                [   0,   0, 255],
            ]
        }
    }
}

impl Block for J {
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
        Box::new(J {
            data: rotate_square_3_right(&self.data),
        })
    }
    fn rotate_left(&self) -> Box<dyn Block> {
        Box::new(J {
            data: rotate_square_3_left(&self.data),
        })
    }
}

