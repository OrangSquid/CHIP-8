use std::{fs, env::{self}, thread};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::cpu::Cpu;

pub mod cpu;
mod ch8display;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let windows = video_subsystem.window("LMAAAAOOOOO", 320, 160).build().unwrap();

    let mut canvas = windows.into_canvas().build().unwrap();

    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut cpu = Cpu::new(canvas);

    match fs::read(path) {
        Ok(x) => cpu.load_into_memory(x),
        Err(_) => panic!("Invalid path")
    }

    println!("{}", cpu.into_string());

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        cpu.next();
    }
}