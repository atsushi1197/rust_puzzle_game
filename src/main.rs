mod func;
mod config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use getch_rs::{Getch, Key};
use config::FIELD;
use config::INITIAL_POSITION;
use config::Direction;
use config::BlockKind;
use func::change_field;
use func::render;
use func::is_touching_wall;
use func::is_reaching_bottom;

use func::is_touching_others;
use func::is_horizontal_collision;
use func::change_block_angle;
use func::erase_full_filled_row;

use func::is_game_over;

use crate::func::calc_score;


fn main() {
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));
    let position = Arc::new(Mutex::new(INITIAL_POSITION));
    let field = Arc::new(Mutex::new(FIELD));
    let score = Arc::new(Mutex::new(0));
    let pause = Arc::new(Mutex::new(false));
    // 画面クリア
    println!("\x1b[2J");
    render(&field.lock().unwrap());
    {
        let position = Arc::clone(&position);
        let block = Arc::clone(&block);
        let field = Arc::clone(&field);
        let pause = Arc::clone(&pause);
        render(&field.lock().unwrap());
        let _ = thread::spawn(move || {
            loop {
                if *pause.lock().unwrap() {
                    continue;
                }
                thread::sleep(Duration::from_millis(2000));
                let mut position = position.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut field = field.lock().unwrap();
                let mut score = score.lock().unwrap();
                if is_game_over(&field) {
                    // 画面クリア
                    println!("\x1b[2J");
                    println!("GAME OVER");
                    println!("YOUR SCORE: {}", *score);
                    // カーソルを再表示
                    println!("\x1b[?25h");
                    return;
                }
                let result = erase_full_filled_row(*field, *score);
                *field = result.0;
                *score = result.1;
                let new_position = position.shift(Direction::Down);
                if is_reaching_bottom(new_position.y, *block) || is_touching_others(&field, &position, *block) {
                    *field = change_field(*field, *block, &position);
                    *block = rand::random();
                    *position = INITIAL_POSITION;
                    render(&field);
                } else {
                    // posの座標を更新
                    *position = new_position;
                    let new_field = change_field(*field, *block, &position);
                    render(&new_field);
                }
                println!("score:{}", *score);
            }
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Down) => {
                let mut position = position.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut field = field.lock().unwrap();
                let new_position = position.shift(Direction::Down);
                if is_reaching_bottom(new_position.y, *block) || is_touching_others(&field, &position, *block) {
                    *field = change_field(*field, *block, &position);
                    *block = rand::random();
                    *position = INITIAL_POSITION;
                    render(&field);
                } else if !is_touching_wall(&new_position.x, *block) {
                    // posの座標を更新
                    *position = new_position;
                    let new_field = change_field(*field, *block, &position);
                    render(&new_field);
                }
            },
            Ok(Key::Left) => {
                let mut position = position.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut field = field.lock().unwrap();
                if position.x > 1 {
                    let new_position = position.shift(Direction::Left);
                    if !is_touching_wall(&new_position.x, *block) {
                        if is_reaching_bottom(new_position.y, *block) || is_touching_others(&field, &position, *block) {
                            *field = change_field(*field, *block, &position);
                            *block = rand::random();
                            *position = INITIAL_POSITION;
                            render(&field);
                        } else if is_horizontal_collision(&field, &position, *block) {
                            render(&field);
                        } else {
                            // posの座標を更新
                            *position = new_position;
                            let new_field = change_field(*field, *block, &position);
                            render(&new_field);
                        }
                    }
                }
            },
            Ok(Key::Right) => {
                let mut position = position.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut field = field.lock().unwrap();
                if position.x + 1 != 13 {
                    let new_position = position.shift(Direction::Right);
                    if !is_touching_wall(&new_position.x, *block) {
                        if is_reaching_bottom(new_position.y, *block) || is_touching_others(&field, &position, *block) {
                            *field = change_field(*field, *block, &position);
                            *block = rand::random();
                            *position = INITIAL_POSITION;
                            render(&field);
                        } else if is_horizontal_collision(&field, &position, *block) {
                            render(&field);
                        } else {
                            // posの座標を更新
                            *position = new_position;
                            let new_field = change_field(*field, *block, &position);
                            render(&new_field);
                        }
                    }
                }
            },
            Ok(Key::Up) => {
                let mut block = block.lock().unwrap();
                *block = change_block_angle(*block);
            },
            Ok(Key::Char('q')) => {
                // カーソルを再表示
                println!("\x1b[?25h");
                return;
            },
            Ok(Key::Char('p')) => {
                let mut pause = pause.lock().unwrap();
                *pause = !(*pause);
            },
            _ => (),
        }
    }
}
