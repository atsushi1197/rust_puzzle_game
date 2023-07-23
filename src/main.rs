mod func;
mod config;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use getch_rs::{Getch, Key};
use config::Position;
// use config::BLOCK_KINDS;
use config::FIELD;
use config::BLOCK_KINDS;
use config::Direction;
use config::INITIAL_POSITION;
use func::change_field;
use func::render;
use func::is_colliding;
use rand::Rng;


fn main() {
    let g = Getch::new();
    let mut rng = rand::thread_rng();
    let mut wanna_quit = false;
    // let mut field = FIELD;
    // 画面クリア
    println!("\x1b[2J");
    loop {
        if wanna_quit {
            break;
        }
        let block = BLOCK_KINDS[rng.gen_range(0..7)];
        // let block = Arc::new(Mutex::new(BLOCK_KINDS[rng.gen_range(0..7)]));
        let position = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
        let new_field = change_field(FIELD, block, &position.lock().unwrap());
        render(&new_field);

        {
            let position = Arc::clone(&position);
            let _ = thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(500));
                    let mut position = position.lock().unwrap();
                    let new_position = position.shift(Direction::Down);
                    if !is_colliding(&FIELD, &new_position, block) {
                        // posの座標を更新
                        *position = new_position;
                    } else {
                        *position = INITIAL_POSITION;
                    }
                    let new_field = change_field(FIELD, block, &position);
                    render(&new_field);
                }
            });
        }
        loop {
            match g.getch() {
                Ok(Key::Down) => {
                    let mut position = position.lock().unwrap();
                    let new_position = position.shift(Direction::Down);
                    if !is_colliding(&FIELD, &new_position, block) {
                        // posの座標を更新
                        *position = new_position;
                    } else {
                        break;
                    }
                },
                Ok(Key::Left) => {
                    if position.lock().unwrap().x - 1 != 0 {
                        let mut position = position.lock().unwrap();
                        let new_position = position.shift(Direction::Left);
                        if !is_colliding(&FIELD, &new_position, block) {
                            // posの座標を更新
                            *position = new_position;
                        } else {
                            break;
                        }
                    }
                },
                Ok(Key::Right) => {
                    if position.lock().unwrap().x + 2 < 11 {
                        let mut position = position.lock().unwrap();
                        let new_position = position.shift(Direction::Right);
                        if !is_colliding(&FIELD, &new_position, block) {
                            // posの座標を更新
                            *position = new_position;
                        } else {
                            break;
                        }
                    }
                },
                Ok(Key::Char('q')) => {
                    wanna_quit = true;
                    break;
                },
                _ => (),
            }

            let new_field = change_field(FIELD, block, &position.lock().unwrap());

            render(&new_field);

            // カーソルを再表示
            println!("\x1b[?25h");
            if (position.lock().unwrap().y) == 21  {
                break;
            }
        }
    }
}
