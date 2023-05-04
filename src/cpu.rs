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

#[derive(Debug)]
pub struct Cpu {
    memory: [u8; 4096],
    pc: u16,
    register_i: u16,
    delay_timer: u8,
    registers: [u8; 16]
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu: Cpu = Cpu {
            memory: [0; 4096],
            pc: 0,
            register_i: 0,
            delay_timer: 0,
            registers: [0; 16]
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

    pub fn run(&self) {

    }
}