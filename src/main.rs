mod func;
mod config;
use std::thread;
use std::time::Duration;
use getch_rs::{Getch, Key};
use config::Position;
// use config::BLOCK_KINDS;
use config::FIELD;
use config::BLOCK_KINDS;
use config::Direction;
use func::change_field;
use func::render;
use func::is_colliding;
use rand::Rng;


fn main() {
    let g = Getch::new();
    let mut rng = rand::thread_rng();
    // 画面クリア
    println!("\x1b[2J");
    loop {
        let block = BLOCK_KINDS[rng.gen_range(0..7)];
        let mut position = Position { x: 4, y: 0 };
        loop {
            // thread::sleep(Duration::from_millis(500));
            match g.getch() {
                Ok(Key::Down) => {
                    let new_pos = position.shift(Direction::Down);
                    if !is_colliding(&FIELD, &new_pos, block) {
                        // posの座標を更新
                        position = new_pos;
                    }
                },
                Ok(Key::Left) => {
                    if position.x - 1 != 0 {
                        let new_pos = position.shift(Direction::Left);
                        if !is_colliding(&FIELD, &new_pos, block) {
                            // posの座標を更新
                            position = new_pos;
                        }
                    }
                },
                Ok(Key::Right) => {
                    if position.x + 2 < 11 {
                        let new_pos = position.shift(Direction::Right);
                        if !is_colliding(&FIELD, &new_pos, block) {
                            // posの座標を更新
                            position = new_pos;
                        }
                    }
                },
                Ok(Key::Char('q')) => break,
                _ => (),
            }

            if !is_colliding(&FIELD, &position, block) {
                // 1マス落下
                position.y += 1;
            } else {
                break;
            }
            let new_field = change_field(FIELD, block, &position);

            render(&new_field);

            // カーソルを再表示
            println!("\x1b[?25h");
            if (position.y) == 21  {
                break;
            }
        }
    }
}
