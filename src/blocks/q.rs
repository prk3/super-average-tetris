use crate::block::Block;

pub struct Q {
}

impl Q {
    pub fn first_rotation() -> Q {
        Q {}
    }
}

impl Block for Q {
    fn width(&self) -> usize {
        2
    }
    fn height(&self) -> usize {
        2
    }
    fn at(&self, _x: usize, _y: usize) -> u8 {
        255
    }
    fn rotate_left(&self) -> Box<dyn Block> {
        Box::new(Q::first_rotation())
    }
    fn rotate_right(&self) -> Box<dyn Block> {
        Box::new(Q::first_rotation())
    }
}
