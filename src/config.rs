use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub const FIELD_WIDTH: usize = 11 + 2;
pub const FIELD_HEIGHT: usize = 20 + 1;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];


#[derive(Clone, Copy)]

pub enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

pub type BlockShape = [[usize; 4]; 4];

pub const BLOCK_KINDS: [BlockShape; 7] = [
    BLOCKS[BlockKind::I as usize],
    BLOCKS[BlockKind::O as usize],
    BLOCKS[BlockKind::S as usize],
    BLOCKS[BlockKind::Z as usize],
    BLOCKS[BlockKind::J as usize],
    BLOCKS[BlockKind::L as usize],
    BLOCKS[BlockKind::T as usize],
];


pub const BLOCKS: [BlockShape; 7] = [
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [3,3,3,3],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,3,0,0],
        [3,3,0,0],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,3,3,0],
        [3,3,0,0],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,3,0,0],
        [0,3,3,0],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,0,0,0],
        [3,3,3,0],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,3,0],
        [3,3,3,0],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [3,3,3,0],
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