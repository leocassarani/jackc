pub enum Instruction {
    A(u16),
    C(CInstruction),
}

pub struct CInstruction(u16);

impl CInstruction {
    pub fn a(&self) -> bool {
        self.bit(12) == 1
    }

    pub fn comp_field(&self) -> [u8; 6] {
        [
            self.bit(11),
            self.bit(10),
            self.bit(9),
            self.bit(8),
            self.bit(7),
            self.bit(6),
        ]
    }

    pub fn d1(&self) -> bool {
        self.bit(5) == 1
    }

    pub fn d2(&self) -> bool {
        self.bit(4) == 1
    }

    pub fn d3(&self) -> bool {
        self.bit(3) == 1
    }

    pub fn j1(&self) -> bool {
        self.bit(2) == 1
    }

    pub fn j2(&self) -> bool {
        self.bit(1) == 1
    }

    pub fn j3(&self) -> bool {
        self.bit(0) == 1
    }

    fn bit(&self, idx: usize) -> u8 {
        (self.0 >> idx & 1) as u8
    }
}

const RAM_SIZE: usize = 16 * 1024; // 32KiB

pub struct Emulator<'a> {
    alu: ALU,
    reg: Registers,
    ram: [u16; RAM_SIZE], // 32KiB
    rom: &'a [Instruction],
    pc: u16,
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &'a [Instruction]) -> Self {
        Emulator {
            alu: ALU::default(),
            reg: Registers::default(),
            ram: [0; 16 * 1024],
            rom,
            pc: 0,
        }
    }

    pub fn run(&mut self, max: usize) {
        for _ in 0..max {
            self.step()
        }
    }

    pub fn step(&mut self) {
        match &self.rom[self.pc as usize] {
            Instruction::A(constant) => self.reg.a = *constant,
            Instruction::C(c_inst) => {
                let x = self.reg.d;

                let y = if c_inst.a() {
                    self.ram[self.reg.a as usize]
                } else {
                    self.reg.a
                };

                self.alu.load(x, y, c_inst.comp_field());

                if self.jump(c_inst) {
                    self.pc = self.reg.a;
                } else {
                    self.pc += 1;
                }

                if c_inst.d1() {
                    self.reg.a = self.alu.out;
                }
                if c_inst.d2() {
                    self.reg.d = self.alu.out;
                }
                if c_inst.d3() {
                    self.ram[self.reg.a as usize] = self.alu.out;
                }
            }
        }
    }

    fn jump(&self, c_inst: &CInstruction) -> bool {
        (c_inst.j1() && self.alu.neg) || (c_inst.j2() && self.alu.zero) || c_inst.j3()
    }
}

#[derive(Default)]
struct ALU {
    out: u16,
    zero: bool,
    neg: bool,
}

impl ALU {
    pub fn load(&mut self, x: u16, y: u16, comp: [u8; 6]) {
        let x = x as i16;
        let y = y as i16;

        let result: i16 = match comp {
            [1, 0, 1, 0, 1, 0] => 0,
            [1, 1, 1, 1, 1, 1] => 1,
            [1, 1, 1, 0, 1, 0] => -1,
            [0, 0, 1, 1, 0, 0] => x,
            [1, 1, 0, 0, 0, 0] => y,
            [0, 0, 1, 1, 0, 1] => !x,
            [1, 1, 0, 0, 0, 1] => !y,
            [0, 0, 1, 1, 1, 1] => -x,
            [1, 1, 0, 0, 1, 1] => -y,
            [0, 1, 1, 1, 1, 1] => x + 1,
            [1, 1, 0, 1, 1, 1] => y + 1,
            [0, 0, 1, 1, 1, 0] => x - 1,
            [1, 1, 0, 0, 1, 0] => y - 1,
            [0, 0, 0, 0, 1, 0] => x + y,
            [0, 1, 0, 0, 1, 1] => x - y,
            [0, 0, 0, 1, 1, 1] => y - x,
            [0, 0, 0, 0, 0, 0] => x & y,
            [0, 1, 0, 1, 0, 1] => x | y,
            _ => panic!("invalid comp fields: {:?}", comp),
        };

        self.out = result as u16;
        self.zero = result == 0;
        self.neg = result < 0;
    }
}

#[derive(Default)]
struct Registers {
    a: u16,
    d: u16,
}
