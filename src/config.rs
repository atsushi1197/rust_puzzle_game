use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub const FIELD_WIDTH: usize = 11 + 2;
pub const FIELD_HEIGHT: usize = 20 + 1;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];


#[derive(Clone, Copy)]
pub enum BlockKind {
    I1,
    O1,
    S1,
    Z1,
    J1,
    L1,
    T1,
    I2,
    S2,
    Z2,
    J2,
    J3,
    J4,
    L2,
    L3,
    L4,
    T2,
    T3,
    T4,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=1) {
            1 => BlockKind::O1,
            2 => BlockKind::S1,
            3 => BlockKind::Z1,
            4 => BlockKind::J1,
            5 => BlockKind::L1,
            _ => BlockKind::T1,
        }
    }
}

pub type BlockShape = [[usize; 4]; 4];

pub const BLOCKS: [BlockShape; 19] = [
    // I1
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [3,3,3,3],
    ],
    // O1
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,3,0,0],
        [3,3,0,0],
    ],
    // S1
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,3,3,0],
        [3,3,0,0],
    ],
    // Z1
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,3,0,0],
        [0,3,3,0],
    ],
    // J1
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,0,0,0],
        [3,3,3,0],
    ],
    // L1
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,3,0],
        [3,3,3,0],
    ],
    // T1
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,3,0,0],
        [3,3,3,0],
    ],
    // I2
    [
        [3,0,0,0],
        [3,0,0,0],
        [3,0,0,0],
        [3,0,0,0],
    ],
    // S2
    [
        [0,0,0,0],
        [3,0,0,0],
        [3,3,0,0],
        [0,3,0,0],
    ],
    // Z2
    [
        [0,0,0,0],
        [0,3,0,0],
        [3,3,0,0],
        [3,0,0,0],
    ],
    // J2
    [
        [0,0,0,0],
        [3,3,0,0],
        [3,0,0,0],
        [3,0,0,0],
    ],
    // J3
    [
        [0,0,0,0],
        [3,3,3,0],
        [0,0,3,0],
        [0,0,0,0],
    ],
    // J4
    [
        [0,0,0,0],
        [0,3,0,0],
        [0,3,0,0],
        [3,3,0,0],
    ],
    // L2
    [
        [0,0,0,0],
        [3,0,0,0],
        [3,0,0,0],
        [3,3,0,0],
    ],
    // L3
    [
        [0,0,0,0],
        [3,3,3,0],
        [3,0,0,0],
        [0,0,0,0],
    ],
    // L4
    [
        [0,0,0,0],
        [3,3,0,0],
        [0,3,0,0],
        [0,3,0,0],
    ],
    // T2
    [
        [0,0,0,0],
        [0,3,0,0],
        [0,3,3,0],
        [0,3,0,0],
    ],
    // T3
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,3,3,0],
        [0,3,0,0],
    ],
    // T2
    [
        [0,0,0,0],
        [0,3,0,0],
        [3,3,0,0],
        [0,3,0,0],
    ],
];

pub struct Position {
    pub x: usize,
    pub y: usize
}

pub const INITIAL_POSITION: Position = Position { x: 5, y: 0};

#[derive(Clone, Copy)]
pub enum Direction {
    Down,
    Left,
    Right
}

impl Position {
    pub fn shift(&self, direction: Direction) -> Position {
        match direction {
            Direction::Down => Position { x: self.x, y: &self.y + 1 },
            Direction::Left => Position { x: &self.x - 1, y: self.y },
            Direction::Right => Position { x: &self.x + 1, y: self.y }
        }
    }
}

pub const FIELD: [[usize; 13]; 21] = [
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,2,2,2,2,2,2,2,2,2,2,2,1],
    ];