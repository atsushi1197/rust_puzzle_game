use std::thread;
use std::time::Duration;

enum BlockKind {
    I,
    // O,
    // S,
    // Z,
    // J,
    // L,
    // T,
}

type BlockShape = [[usize; 4]; 4];

const BLOCKS: [BlockShape; 7] = [
    [
        [3,3,3,3],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    [
        [0,3,3,0],
        [0,3,3,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    [
        [0,3,3,0],
        [3,3,0,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    [
        [3,3,0,0],
        [0,3,3,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    [
        [3,0,0,0],
        [3,3,3,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
    [
        [0,0,0,3],
        [0,3,3,3],
        [0,0,0,0],
        [0,0,0,0],
    ],
    [
        [0,3,3,3],
        [0,0,3,0],
        [0,0,0,0],
        [0,0,0,0],
    ],
];

struct Position {
    x: usize,
    y: usize
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

    let mut position = Position { x: 1, y: 0 };

    // 画面クリア
    println!("\x1b[2J");

    for _ in 0..21 {
        thread::sleep(Duration::from_secs(1));
        // フィールドの状態を初期化
        let mut field_buffer = field;
        // 落下するブロックを表示するためにブロック位置のデータを書き換え処理
        for y in 0..4 {
            for x in 0..4 {
                // field_buffer[y+2][x+2] = BLOCKS[BlockKind::I as usize][y][x];
                // field_buffer[y+2][x+7] = BLOCKS[BlockKind::O as usize][y][x];
                // field_buffer[y+6][x+2] = BLOCKS[BlockKind::S as usize][y][x];
                // field_buffer[y+6][x+7] = BLOCKS[BlockKind::Z as usize][y][x];
                // field_buffer[y+10][x+2] = BLOCKS[BlockKind::J as usize][y][x];
                // field_buffer[y+10][x+7] = BLOCKS[BlockKind::L as usize][y][x];
                // field_buffer[y+14][x+2] = BLOCKS[BlockKind::T as usize][y][x];
                if BLOCKS[BlockKind::I as usize][y][x] == 3 {
                    field_buffer[y + position.y][x + position.x] = 3;
                }
            }
        }
        // 1マス落下
        position.y += 1;

        // カーソルを先頭に移動
        println!("\x1b[H");

        // field_bufferを見て、そのデータに従って、マスの表示を切り替え
        for y in 0..21 {
            // if (&position.y + &y) > 20  {
            //     break;
            // }
            for x in 0..13 {
                if field_buffer[y][x] == 1 {
                    print!("|");
                } else if field_buffer[y][x] == 2 {
                    print!("__");
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
    }
}
