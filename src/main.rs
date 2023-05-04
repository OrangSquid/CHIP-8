use std::{fs, env::{self}};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::cpu::Cpu;

pub mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut cpu = Cpu::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let windows = video_subsystem.window("LMAAAAOOOOO", 320, 160).build().unwrap();

    let mut canvas = windows.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(Rect::new(0, 0, 5, 5));

    canvas.present();

    match fs::read(path) {
        Ok(x) => cpu.load_into_memory(x),
        Err(_) => panic!("Invalid path")
    }

    println!("{}", cpu.into_string());
}
