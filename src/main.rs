extern crate time;

extern crate piston;
extern crate sdl2_game_window;

use std::io::timer::sleep;

use time::precise_time_ns;

use sdl2_game_window::GameWindowSDL2;
use piston::{
    GameIteratorSettings,
    GameIterator,
    GameWindowSettings,
    Render,
    Update,
};

fn main() {
    let mut window = GameWindowSDL2::new(
        GameWindowSettings {
            title: "Window".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
        );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };

    let mut game_iter = GameIterator::new( &mut window, &game_iter_settings );

    let mut time = precise_time_ns();

    let mut times = vec!();

    for e in game_iter {
        match e {
            Render(_) => {
                times.push(format!("Render: {}", (precise_time_ns() - time)/1000000));
                if times.len() >= 100 {
                    break;
                }
                time = precise_time_ns();
            }
            Update(_) => {
                times.push(format!("Update: {}", (precise_time_ns() - time)/1000000));
                sleep(6);
                time = precise_time_ns();
            }
            _ => {}
        }
    }

    for s in times.iter() {
        println!("{}", s);
    }
}
