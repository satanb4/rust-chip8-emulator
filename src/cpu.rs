use crate::ram::Ram;
use std::fmt;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
        }
    }

    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    fn debug_draw_sprite(&self, ram: &mut Ram, x: u8, y: u8, height: u8) {
        // Draws sprite of coordinate Vx,Vy with width 8 pixels and height `n`
        println!("Drawing sprite at ({},{})", x, y);
        for y in 0..height {
            let mut b = ram.read_byte(self.i + y as u16);
            for _ in 0..b {
                match (b & 0b1000_0000) >> 7 {
                    0 => print!(" "),
                    1 => print!("#"),
                    _ => unreachable!(),
                }
                b = b << 1;
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction = (hi << 8) | lo;
        println!(
            "Instruction read {:#x}: hi-{:#x} lo-{:#x} ",
            instruction, hi, lo
        );

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;
        println!(
            "nnn={:#X},nn={:#X},n={:#X},x={:#X},y={:#X}",
            nnn, nn, n, x, y
        );

        if self.prev_pc == self.pc {
            println!("Please increment PC!");
            panic!();
        }
        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                // 1NNN: goto nnn
                self.pc = nnn;
            }
            0x3 => {
                // 3XNN: if(Vx==NN)
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                // 6XNN: set vx to nn
                self.write_reg_vx(x, nn);
                self.pc += 2;
            }
            0x7 => {
                // 7XNN: Adds nn to vx (no carry flag change)
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            }
            0xD => {
                // DXYN: Draws sprite
                self.debug_draw_sprite(ram, x, y, n);
                self.pc += 2
            }
            0xA => {
                // ANNN: I = NNN
                self.i = nnn;
                self.pc += 2;
            }
            0xF => {
                // FX1E: Adds Vx to I (I += Vx)
                let vx = self.read_reg_vx(x);
                self.i += vx as u16;
                self.pc += 2;
            }

            _ => panic!(
                "Unrecognized Instruction passed {:#x}-{:#x}",
                self.pc, instruction
            ),
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc: {:#X}\n", self.pc);
        write!(f, "vx: ");
        for item in self.vx.iter() {
            write!(f, "{:#x} ", *item);
        }
        write!(f, "\n");
        write!(f, "i: {:#X}\n", self.i)
    }
}
