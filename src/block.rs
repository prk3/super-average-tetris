
pub trait Block {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at(&self, x: usize, y: usize) -> u8;
    fn rotate_left(&self) -> Box<dyn Block>;
    fn rotate_right(&self) -> Box<dyn Block>;
}

