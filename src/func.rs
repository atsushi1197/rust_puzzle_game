use crate::config::BlockKind;
use crate::config::FIELD_WIDTH;
use crate::config::FIELD_HEIGHT;
use crate::config::BLOCKS;
use crate::config::Position;
use crate::config::Field;

pub fn change_field(mut field: Field, block: BlockKind, position: &Position) -> Field {
    // let mut new_field = field;
    // 落下するブロックを表示するためにブロック位置のデータを書き換え処理
    for y in (0..4).rev() {
        for x in 0..4 {
            if x + position.x >= 12 || y + position.y >= 20 {
                continue;
            }
            if BLOCKS[block as usize][y][x] == 3 {
                field[y + position.y][x + position.x] = 3;
            }
        }
    }
    field
}

pub fn render(field: &Field) {
    // カーソルを先頭に移動
    println!("\x1b[H");
    // field_bufferを見て、そのデータに従って、マスの表示を切り替え
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field[y][x] == 1 {
                print!("|");
            } else if field[y][x] == 2 {
                print!("##");
            } else if field[y][x] == 3 {
                print!("🔳");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

pub fn is_touching_wall(position_x: &usize, block: BlockKind) -> bool {
    let mut right_end = 0;
    for x in 0..4 {
        for y in 0..4 {
            if BLOCKS[block as usize][y][x] == 3 && x > right_end {
                right_end = x;
            }
        }
    }
    if position_x + right_end >= 12 {
        return true;
    }
    false
}

pub fn is_reaching_bottom(new_position_y: usize, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if  BLOCKS[block as usize][y][x] == 3 && (new_position_y + y) == FIELD_HEIGHT - 1 {
                return true;
            }
        }
    }
    false
}

pub fn is_touching_others(field: &Field, current_position: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[block as usize][y][x] & field[current_position.y + y + 1][current_position.x + x] == 3 {
                return true;
            }
        }
    }
    false
}
