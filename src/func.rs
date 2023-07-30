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
            if current_position.y + y + 1 > 20 || current_position.x + x > 12 {
                continue;
            }
            if BLOCKS[block as usize][y][x] & field[current_position.y + y + 1][current_position.x + x] == 3 {
                return true;
            }
        }
    }
    false
}

pub fn is_horizontal_collision(field: &Field, current_position: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if current_position.x + x + 1 > 12 {
                continue;
            }
            if BLOCKS[block as usize][y][x] & field[current_position.y + y][current_position.x + x - 1] == 3
            || BLOCKS[block as usize][y][x] & field[current_position.y + y][current_position.x + x + 1] == 3
            {
                return true;
            }
        }
    }
    false
}

pub fn change_block_angle(current_block: BlockKind) -> BlockKind {
    match current_block {
        BlockKind::I1 => BlockKind::I2,
        BlockKind::O1 => BlockKind::O1,
        BlockKind::S1 => BlockKind::S2,
        BlockKind::Z1 => BlockKind::Z2,
        BlockKind::J1 => BlockKind::J2,
        BlockKind::L1 => BlockKind::L2,
        BlockKind::T1 => BlockKind::T2,
        BlockKind::I2 => BlockKind::I1,
        BlockKind::S2 => BlockKind::S1,
        BlockKind::Z2 => BlockKind::Z1,
        BlockKind::J2 => BlockKind::J3,
        BlockKind::L2 => BlockKind::L3,
        BlockKind::T2 => BlockKind::T3,
        BlockKind::J3 => BlockKind::J4,
        BlockKind::L3 => BlockKind::L4,
        BlockKind::T3 => BlockKind::T4,
        BlockKind::J4 => BlockKind::J1,
        BlockKind::L4 => BlockKind::L1,
        BlockKind::T4 => BlockKind::T1,
    }
}

pub fn erase_full_filled_row(current_field: Field) -> (Field, usize) {
    let mut new_field = current_field;
    let mut erased_count = 0;
    for y in (0..(FIELD_HEIGHT - 1)).rev() {
        if y - erased_count == 0 {
            break;
        }
        for x in 1..(FIELD_WIDTH - 1) {
            if current_field[y - erased_count][x] == 0 {
                new_field[y] = current_field[y - erased_count];
                break;
            } else if x >= FIELD_WIDTH - 3 {
                erased_count += 1;
                new_field[y] = current_field[y - erased_count];
            }
        }
    }
    (new_field, erased_count)
}

pub fn is_game_over(current_field: &Field) -> bool {
    for x in 1..FIELD_WIDTH - 1 {
        if current_field[2][x] == 3 {
            return true;
        }
    }
    false
}

pub fn calc_score(current_score: usize ,erased_count: usize) -> usize {
    match erased_count {
        0 => current_score,
        1 => current_score + 10,
        2 => current_score + 30,
        3 => current_score + 90,
        4 => current_score + 160,
        _ => current_score,
    }
}