const L_ROTATIONS: [u16; 4] = [
    0b_0000_1110_1000_0000,
    0b_1100_0100_0100_0000,
    0b_0010_1110_0000_0000,
    0b_0100_0100_0110_0000,
];

const J_ROTATIONS: [u16; 4] = [
    0b_0000_1110_0010_0000,
    0b_0100_0100_1100_0000,
    0b_1000_1110_0000_0000,
    0b_0110_0100_0100_0000,
];

const S_ROTATIONS: [u16; 4] = [
    0b_0000_0110_1100_0000,
    0b_0100_0110_0010_0000,
    0b_0000_0110_1100_0000,
    0b_0100_0110_0010_0000,
];

const Z_ROTATIONS: [u16; 4] = [
    0b_0000_1100_0110_0000,
    0b_0100_1100_1000_0000,
    0b_0000_1100_0110_0000,
    0b_0100_1100_1000_0000,
];

const T_ROTATIONS: [u16; 4] = [
    0b_0000_1110_0100_0000,
    0b_0100_1100_0100_0000,
    0b_0100_1110_0000_0000,
    0b_0100_0110_0100_0000,
];

const I_ROTATIONS: [u16; 4] = [
    0b_0000_0000_1111_0000,
    0b_0010_0010_0010_0010,
    0b_0000_0000_1111_0000,
    0b_0010_0010_0010_0010,
];

const Q_ROTATIONS: [u16; 4] = [
    0b_1100_1100_0000_0000,
    0b_1100_1100_0000_0000,
    0b_1100_1100_0000_0000,
    0b_1100_1100_0000_0000,
];

#[derive(Clone, Copy)]
enum Rotation { Zero, One, Two, Three }

#[derive(Clone, Copy)]
pub enum BlockType { L, J, S, Z, T, I, Q }

#[derive(Clone)]
pub struct Block {
    block_type: BlockType,
    rotation: Rotation,
    data: u16,
    start_offset_x: i8,
    start_offset_y: i8,
}

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        let rotation = Rotation::Zero;

        match block_type {
            BlockType::L => Block {
                block_type,
                rotation,
                data: L_ROTATIONS[0],
                start_offset_x: -1,
                start_offset_y: -1,
            },
            BlockType::J => Block {
                block_type,
                rotation,
                data: J_ROTATIONS[0],
                start_offset_x: -1,
                start_offset_y: -1,
            },
            BlockType::S => Block {
                block_type,
                rotation,
                data: S_ROTATIONS[0],
                start_offset_x: -1,
                start_offset_y: -1,
            },
            BlockType::Z => Block {
                block_type,
                rotation,
                data: Z_ROTATIONS[0],
                start_offset_x: -1,
                start_offset_y: -1,
            },
            BlockType::T => Block {
                block_type,
                rotation,
                data: T_ROTATIONS[0],
                start_offset_x: -1,
                start_offset_y: -1,
            },
            BlockType::I => Block {
                block_type,
                rotation,
                data: I_ROTATIONS[0],
                start_offset_x: -2,
                start_offset_y: -2,
            },
            BlockType::Q => Block {
                block_type,
                rotation,
                data: Q_ROTATIONS[0],
                start_offset_x: -1,
                start_offset_y: 0,
            }
        }
    }

    pub fn at(&self, x: usize, y: usize) -> u8 {
        if self.data & (0b_1000_0000_0000_0000 >> ((y * 4) + x)) != 0 {
            255
        } else {
            0
        }
    }

    pub fn start_offset_x(&self) -> i8 {
        self.start_offset_x
    }

    pub fn start_offset_y(&self) -> i8 {
        self.start_offset_y
    }

    pub fn rotate_left(&self) -> Block {
        let new_rotation = match self.rotation {
            Rotation::Zero  => Rotation::Three,
            Rotation::One   => Rotation::Zero,
            Rotation::Two   => Rotation::One,
            Rotation::Three => Rotation::Two,
        };
        self.make_rotated(new_rotation)
    }

    pub fn rotate_right(&self) -> Block {
        let new_rotation = match self.rotation {
            Rotation::Zero  => Rotation::One,
            Rotation::One   => Rotation::Two,
            Rotation::Two   => Rotation::Three,
            Rotation::Three => Rotation::Zero,
        };
        self.make_rotated(new_rotation)
    }

    fn make_rotated(&self, rotation: Rotation) -> Block {
        let rotation_index = match rotation {
            Rotation::Zero  => 0,
            Rotation::One   => 1,
            Rotation::Two   => 2,
            Rotation::Three => 3,
        };

        Block {
            rotation,
            data: match self.block_type {
                BlockType::L => L_ROTATIONS[rotation_index],
                BlockType::J => J_ROTATIONS[rotation_index],
                BlockType::S => S_ROTATIONS[rotation_index],
                BlockType::Z => Z_ROTATIONS[rotation_index],
                BlockType::T => T_ROTATIONS[rotation_index],
                BlockType::I => I_ROTATIONS[rotation_index],
                BlockType::Q => Q_ROTATIONS[rotation_index],
            },
            ..*self
        }
    }
}
