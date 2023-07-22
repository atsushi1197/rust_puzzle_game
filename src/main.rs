use std::thread;
use std::time::Duration;

const FIELD_WIDTH: usize = 11 + 2;
const FIELD_HEIGHT: usize = 20 + 1;
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone, Copy)]

enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

const BLOCK_KINDS: [[[usize; 4]; 4]; 7] = [
    BLOCKS[BlockKind::O as usize],
    BLOCKS[BlockKind::I as usize],
    BLOCKS[BlockKind::S as usize],
    BLOCKS[BlockKind::Z as usize],
    BLOCKS[BlockKind::J as usize],
    BLOCKS[BlockKind::L as usize],
    BLOCKS[BlockKind::T as usize],
];

type BlockShape = [[usize; 4]; 4];

const BLOCKS: [BlockShape; 7] = [
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [3,3,3,3],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,3,3,0],
        [0,3,3,0],
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
        [0,0,0,3],
        [0,3,3,3],
    ],
    [
        [0,0,0,0],
        [0,0,0,0],
        [0,3,3,3],
        [0,0,3,0],
    ],
];

struct Position {
    x: usize,
    y: usize
}

// ブロックがフィールドに衝突する場合は`true`を返す
fn is_colliding(field: &Field, position: &Position, block: [[usize; 4]; 4]) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + position.y + 1][x + position.x] & block[y][x] != 0 {
                return true;
            }
        }
    }
    false
}


fn main() {
    let field = [
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


    // 画面クリア
    println!("\x1b[2J");
    for block in BLOCK_KINDS {
        let mut position = Position { x: 1, y: 0 };
        loop {
            thread::sleep(Duration::from_millis(500));
            // フィールドの状態を初期化
            let mut field_buffer = field;

            if !is_colliding(&field, &position, block) {
                // 1マス落下
                position.y += 1;
            } else {
                break;
            }
            // 落下するブロックを表示するためにブロック位置のデータを書き換え処理
            for y in (0..4).rev() {
                for x in 0..4 {
                    if block[y][x] == 3 {
                        field_buffer[y + position.y][x + position.x] = 3;
                    }
                }
            }

            // カーソルを先頭に移動
            println!("\x1b[H");

            // field_bufferを見て、そのデータに従って、マスの表示を切り替え
            for y in 0..FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    if field_buffer[y][x] == 1 {
                        print!("|");
                    } else if field_buffer[y][x] == 2 {
                        print!("##");
                    } else if field_buffer[y][x] == 3 {
                        print!("🔳");
                    } else {
                        print!("  ");
                    }
                }
                println!();
            }
            // カーソルを再表示
            println!("\x1b[?25h");
            if (position.y) == 21  {
                break;
            }
        }
    }
}
