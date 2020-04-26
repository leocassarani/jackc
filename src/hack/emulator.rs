const RAM_SIZE: usize = 16 * 1024; // 32KiB

pub struct Emulator<'a> {
    pub ram: RAM,
    alu: ALU,
    reg: Registers,
    rom: &'a [u16],
    pc: u16,
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &'a [u16]) -> Self {
        Emulator {
            ram: RAM::new(),
            alu: ALU::default(),
            reg: Registers::default(),
            rom,
            pc: 0,
        }
    }

    pub fn run(&mut self, ticks: usize) {
        for _ in 0..ticks {
            self.step()
        }
    }

    pub fn step(&mut self) {
        if let Some(inst) = self.rom.get(self.pc as usize) {
            if inst >> 15 == 0u16 {
                self.reg.a = *inst;
                self.pc += 1;
            } else {
                let c_inst = CInstruction(*inst);

                let addr = self.reg.a;
                let x = self.reg.d;

                let y = if c_inst.a() {
                    self.ram.get(self.reg.a)
                } else {
                    self.reg.a
                };

                self.alu.load(x, y, c_inst.comp());

                if c_inst.d1() {
                    self.reg.a = self.alu.out;
                }
                if c_inst.d2() {
                    self.reg.d = self.alu.out;
                }
                if c_inst.d3() {
                    self.ram.set(addr, self.alu.out);
                }

                if self.jump(&c_inst) {
                    self.pc = addr;
                } else {
                    self.pc += 1;
                }
            }
        }
    }

    fn jump(&self, c_inst: &CInstruction) -> bool {
        (c_inst.j1() && self.alu.neg)
            || (c_inst.j2() && self.alu.zero)
            || (c_inst.j3() && self.alu.pos)
    }
}

pub struct RAM([u16; RAM_SIZE]);

impl RAM {
    fn new() -> Self {
        Self([0; RAM_SIZE])
    }

    pub fn init(&mut self, map: &[(u16, u16)]) {
        for (addr, val) in map.iter() {
            self.0[*addr as usize] = *val;
        }
    }

    pub fn get(&self, addr: u16) -> u16 {
        self.0[addr as usize]
    }

    pub fn set(&mut self, addr: u16, val: u16) {
        self.0[addr as usize] = val;
    }
}

#[derive(Default)]
struct ALU {
    out: u16,
    zero: bool,
    neg: bool,
    pos: bool,
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
        self.pos = result > 0;
    }
}

#[derive(Debug, Default)]
struct Registers {
    a: u16,
    d: u16,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CInstruction(u16);

impl CInstruction {
    pub fn a(&self) -> bool {
        self.bit(12) == 1
    }

    pub fn comp(&self) -> [u8; 6] {
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
