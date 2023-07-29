mod func;
mod config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use getch_rs::{Getch, Key};
use config::Position;
use config::FIELD;
use config::Direction;
use config::BlockKind;
// use config::INITIAL_POSITION;
use func::change_field;
use func::render;
use func::is_touching_wall;
use func::is_reaching_bottom;

use crate::config::INITIAL_POSITION;
use crate::func::is_touching_others;


fn main() {
    let block = Arc::new(Mutex::new(rand::random::<BlockKind>()));
    let position = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let field = Arc::new(Mutex::new(FIELD));
    // 画面クリア
    println!("\x1b[2J");
    render(&field.lock().unwrap());
    {
        let position = Arc::clone(&position);
        let block = Arc::clone(&block);
        let field = Arc::clone(&field);
        render(&field.lock().unwrap());
        let _ = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1000));
                let mut position = position.lock().unwrap();
                let mut block = block.lock().unwrap();
                let mut field = field.lock().unwrap();
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
            }
        });
    }

    let g = Getch::new();
    loop {
        render(&field.lock().unwrap());
        match g.getch() {
            Ok(Key::Down) => {
                render(&field.lock().unwrap());
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
                if position.x - 1 != 0 {
                    let new_position = position.shift(Direction::Left);
                    if !is_touching_wall(&new_position.x, *block) {
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
                        } else {
                            // posの座標を更新
                            *position = new_position;
                            let new_field = change_field(*field, *block, &position);
                            render(&new_field);
                        }
                    }
                }
            },
            Ok(Key::Char('q')) => {
                // カーソルを再表示
                println!("\x1b[?25h");
                return;
            },
            _ => (),
        }

        render(&field.lock().unwrap());
    }
}
