use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

use crate::ch8display::CH8Display;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
const PROGRAM_START: usize = 0x200;

pub struct Cpu<T: RenderTarget> {
    memory: [u8; 4096],
    pc: u16,
    register_i: u16,
    delay_timer: u8,
    registers: [u8; 16],
    stack: Vec<u16>,
    windows_display: CH8Display<T>
}

impl<T: RenderTarget> Cpu<T> {
    pub fn new(mut canvas: Canvas<T>) -> Cpu<T> {
        let mut cpu = Cpu {
            memory: [0; 4096],
            pc: 0x200,
            register_i: 0,
            delay_timer: 0,
            registers: [0; 16],
            stack: vec![],
            windows_display: CH8Display::new(canvas)
        };
        cpu.memory[0x50..0xA0].copy_from_slice(&FONT);
        cpu
    }

    pub fn load_into_memory(&mut self, bytes: Vec<u8>) {
        self.memory[PROGRAM_START..(PROGRAM_START + bytes.len())].copy_from_slice(&bytes);
    }

    pub fn into_string(&self) -> String {
        let mut result = String::with_capacity(4096 * 3 + (4096 / 16) * 3);
        for (j, i) in self.memory.into_iter().enumerate() {
            if j % 16 == 0 {
                result.push_str(&format!("\n {:#06X} |", j));
            }
            result.push_str(&format!("{:02X} ", i));
        }
        result
    }

    pub fn next(&mut self) {
        let opcode = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        self.decode(opcode);
    }

    fn decode(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x0 => 
            match opcode & 0xFF {
                0xE0 => self.clear_display(),
                0xEE => (),
                _ => ()
            }
            0x1000 => self.jump(opcode & 0xFFF),
            0x6000 => self.set_register(((opcode & 0xF00) >> 8) as u8, (opcode & 0xFF) as u8),
            0x7000 => self.add_to_register(((opcode & 0xF00) >> 8) as u8, (opcode & 0xFF) as u8),
            0xA000 => self.set_register_i(opcode & 0xFFF),
            0xD000 => self.display_sprite(((opcode & 0xF00) >> 8) as u8, ((opcode & 0xF0) >> 4) as u8, (opcode & 0xF) as u8),
            _ => ()
        }
    }

    fn clear_display(&mut self) {
        self.windows_display.clear();
    }

    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn set_register(&mut self, register: u8, value: u8) {
        self.registers[register as usize] = value;
    }

    fn add_to_register(&mut self, register: u8, value: u8) {
        self.registers[register as usize] += value;
    }

    fn set_register_i(&mut self, value: u16) {
        self.register_i = value;
    }

    fn display_sprite(&mut self, register_x: u8, register_y: u8, sprite_height: u8) {
        let x = self.registers[register_x as usize] % 64;
        let y = self.registers[register_y as usize] % 32;
        self.registers[0xF] = 0;
        let mut sprite: Vec<u8> = Vec::with_capacity(sprite_height as usize);
        for i in 0..sprite_height {
            let nth_row = self.memory[(self.register_i + i as u16) as usize];
            sprite.push(nth_row);
        }
        self.windows_display.draw_sprite(x as usize, y as usize, sprite);
    }
}