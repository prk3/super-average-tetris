use crate::block::Block;
use super::rotate::{rotate_square_3_left, rotate_square_3_right};

pub struct L {
    data: [[u8; 3]; 3],
}

impl L {
    pub fn first_rotation() -> L {
        L {
            data: [
                [   0,   0,   0],
                [ 255, 255, 255],
                [ 255,   0,   0],
            ]
        }
    }
}

impl Block for L {
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
        Box::new(L {
            data: rotate_square_3_right(&self.data),
        })
    }
    fn rotate_left(&self) -> Box<dyn Block> {
        Box::new(L {
            data: rotate_square_3_left(&self.data),
        })
    }
}
