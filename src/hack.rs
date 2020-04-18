use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    A(u16),
    C(CInstruction),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('@') {
            let constant = match &s[1..] {
                "SP" | "R0" => Ok(0),
                "LCL" | "R1" => Ok(1),
                "ARG" | "R2" => Ok(2),
                "THIS" | "R3" => Ok(3),
                "THAT" | "R4" => Ok(4),
                "R5" => Ok(5),
                "R6" => Ok(6),
                "R7" => Ok(7),
                "R8" => Ok(8),
                "R9" => Ok(9),
                "R10" => Ok(10),
                "R11" => Ok(11),
                "R12" => Ok(12),
                "R13" => Ok(13),
                "R14" => Ok(14),
                "R15" => Ok(15),
                "SCREEN" => Ok(0x4000),
                "KBD" => Ok(0x6000),
                number => number.parse().or(Err(())),
            };
            constant.map(Instruction::A)
        } else if s.contains('=') {
            match s.splitn(2, '=').collect::<Vec<_>>()[..] {
                [lhs, rhs] => {
                    let (d1, d2, d3) = match lhs {
                        "M" => Ok((0, 0, 1)),
                        "D" => Ok((0, 1, 0)),
                        "MD" => Ok((0, 1, 1)),
                        "A" => Ok((1, 0, 0)),
                        "AM" => Ok((1, 0, 1)),
                        "AD" => Ok((1, 1, 0)),
                        "AMD" => Ok((1, 1, 1)),
                        _ => Err(()),
                    }?;

                    let (a, c1, c2, c3, c4, c5, c6) = match rhs {
                        "0" => Ok((0, 1, 0, 1, 0, 1, 0)),
                        "1" => Ok((0, 1, 1, 1, 1, 1, 1)),
                        "-1" => Ok((0, 1, 1, 1, 0, 1, 0)),
                        "D" => Ok((0, 0, 0, 1, 1, 0, 0)),
                        "A" => Ok((0, 1, 1, 0, 0, 0, 0)),
                        "!D" => Ok((0, 0, 0, 1, 1, 0, 1)),
                        "!A" => Ok((0, 1, 1, 0, 0, 0, 1)),
                        "-D" => Ok((0, 0, 0, 1, 1, 1, 1)),
                        "-A" => Ok((0, 1, 1, 0, 0, 1, 1)),
                        "D+1" => Ok((0, 0, 1, 1, 1, 1, 1)),
                        "A+1" => Ok((0, 1, 1, 0, 1, 1, 1)),
                        "D-1" => Ok((0, 0, 0, 1, 1, 1, 0)),
                        "A-1" => Ok((0, 1, 1, 0, 0, 1, 0)),
                        "D+A" => Ok((0, 0, 0, 0, 0, 1, 0)),
                        "D-A" => Ok((0, 0, 1, 0, 0, 1, 1)),
                        "A-D" => Ok((0, 0, 0, 0, 1, 1, 1)),
                        "D&A" => Ok((0, 0, 0, 0, 0, 0, 0)),
                        "D|A" => Ok((0, 0, 1, 0, 1, 0, 1)),
                        "M" => Ok((1, 1, 1, 0, 0, 0, 0)),
                        "!M" => Ok((1, 1, 1, 0, 0, 0, 1)),
                        "-M" => Ok((1, 1, 1, 0, 0, 1, 1)),
                        "M+1" => Ok((1, 1, 1, 0, 1, 1, 1)),
                        "M-1" => Ok((1, 1, 1, 0, 0, 1, 0)),
                        "D+M" => Ok((1, 0, 0, 0, 0, 1, 0)),
                        "D-M" => Ok((1, 0, 1, 0, 0, 1, 1)),
                        "M-D" => Ok((1, 0, 0, 0, 1, 1, 1)),
                        "D&M" => Ok((1, 0, 0, 0, 0, 0, 0)),
                        "D|M" => Ok((1, 0, 1, 0, 1, 0, 1)),
                        _ => Err(()),
                    }?;

                    let inst = 0b1110000000000000
                        | a << 12
                        | c1 << 11
                        | c2 << 10
                        | c3 << 9
                        | c4 << 8
                        | c5 << 7
                        | c6 << 6
                        | d1 << 5
                        | d2 << 4
                        | d3 << 3;

                    Ok(Instruction::C(CInstruction(inst)))
                }
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CInstruction(pub u16);

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
    pub ram: RAM,

    alu: ALU,
    reg: Registers,
    rom: &'a [Instruction],
    pc: u16,
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &'a [Instruction]) -> Self {
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
        match self.rom.get(self.pc as usize) {
            Some(Instruction::A(constant)) => {
                self.reg.a = *constant;
                self.pc += 1;
            }
            Some(Instruction::C(c_inst)) => {
                let addr = self.reg.a;

                let x = self.reg.d;

                let y = if c_inst.a() {
                    self.ram.get(self.reg.a)
                } else {
                    self.reg.a
                };

                self.alu.load(x, y, c_inst.comp_field());

                if c_inst.d1() {
                    self.reg.a = self.alu.out;
                }
                if c_inst.d2() {
                    self.reg.d = self.alu.out;
                }
                if c_inst.d3() {
                    self.ram.set(addr, self.alu.out);
                }

                if self.jump(c_inst) {
                    self.pc = addr;
                } else {
                    self.pc += 1;
                }
            }
            _ => {} // If we run out of ROM, ignore it
        }
    }

    fn jump(&self, c_inst: &CInstruction) -> bool {
        (c_inst.j1() && self.alu.neg) || (c_inst.j2() && self.alu.zero) || c_inst.j3()
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
