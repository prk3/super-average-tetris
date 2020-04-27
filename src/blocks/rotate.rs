
pub fn rotate_square_3_right<T: Copy>(s: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    [
        [s[2][0], s[1][0], s[0][0]],
        [s[2][1], s[1][1], s[0][1]],
        [s[2][2], s[1][2], s[0][2]],
    ]
}

pub fn rotate_square_3_left<T: Copy>(s: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    [
        [s[0][2], s[1][2], s[2][2]],
        [s[0][1], s[1][1], s[2][1]],
        [s[0][0], s[1][0], s[2][0]],
    ]
}

pub enum Rotation2 { R0, R1 }
