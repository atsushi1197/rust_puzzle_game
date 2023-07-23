use crate::config::FIELD_WIDTH;
use crate::config::FIELD_HEIGHT;
use crate::config::Position;
use crate::config::Field;
use std::thread;
use std::time::Duration;


pub fn change_field(field_buffer: Field, block: [[usize; 4]; 4], position: &Position) -> Field {
    let mut new_field: Field = field_buffer;
    // 落下するブロックを表示するためにブロック位置のデータを書き換え処理
    for y in (0..4).rev() {
        for x in 0..4 {
            if block[y][x] == 3 {
                new_field[y + position.y][x + position.x] = 3;
            }
        }
    }
    new_field
}

pub fn render(field_buffer: &Field) {
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
}

pub fn is_colliding(field: &Field, position: &Position, block: [[usize; 4]; 4]) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if field[y + position.y][x + position.x] & block[y][x] != 0 {
                return true;
            }
        }
    }
    false
}
