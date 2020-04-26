use super::instruction::*;
use std::collections::HashMap;

const DEFAULT_SYMBOLS: &[(&str, u16)] = &[
    ("SP", 0),
    ("LCL", 1),
    ("ARG", 2),
    ("THIS", 3),
    ("THAT", 4),
    ("R0", 0),
    ("R1", 1),
    ("R2", 2),
    ("R3", 3),
    ("R4", 4),
    ("R5", 5),
    ("R6", 6),
    ("R7", 7),
    ("R8", 8),
    ("R9", 9),
    ("R10", 10),
    ("R11", 11),
    ("R12", 12),
    ("R13", 13),
    ("R14", 14),
    ("R15", 15),
    ("SCREEN", 0x4000),
    ("KBD", 0x6000),
];

pub fn assemble(prog: &[Instruction]) -> Vec<u16> {
    let mut symbols: HashMap<String, u16> = DEFAULT_SYMBOLS
        .into_iter()
        .map(|&(sym, idx)| (sym.to_owned(), idx))
        .collect();

    let mut unlabelled = Vec::new();
    let mut idx = 0;

    prog.iter().for_each(|instr| match instr {
        Instruction::A(_) | Instruction::C(_, _, _) => {
            unlabelled.push(instr);
            idx += 1;
        }
        Instruction::Label(label) => {
            symbols.insert(label.to_owned(), idx); // TODO: duplicate entries
            ()
        }
    });

    unlabelled
        .iter()
        .map(|instr| match instr {
            Instruction::A(Load::Constant(n)) => *n,
            Instruction::A(Load::Symbol(symbol)) => symbols[symbol],
            Instruction::C(dest, comp, jump) => {
                let (d1, d2, d3) = match dest {
                    None => (0, 0, 0),
                    Some(Dest::M) => (0, 0, 1),
                    Some(Dest::D) => (0, 1, 0),
                    Some(Dest::MD) => (0, 1, 1),
                    Some(Dest::A) => (1, 0, 0),
                    Some(Dest::AM) => (1, 0, 1),
                    Some(Dest::AD) => (1, 1, 0),
                    Some(Dest::AMD) => (1, 1, 1),
                };

                let (a, c1, c2, c3, c4, c5, c6) = match comp {
                    Comp::Zero => (0, 1, 0, 1, 0, 1, 0),
                    Comp::One => (0, 1, 1, 1, 1, 1, 1),
                    Comp::NegOne => (0, 1, 1, 1, 0, 1, 0),
                    Comp::D => (0, 0, 0, 1, 1, 0, 0),
                    Comp::A => (0, 1, 1, 0, 0, 0, 0),
                    Comp::NotD => (0, 0, 0, 1, 1, 0, 1),
                    Comp::NotA => (0, 1, 1, 0, 0, 0, 1),
                    Comp::NegD => (0, 0, 0, 1, 1, 1, 1),
                    Comp::NegA => (0, 1, 1, 0, 0, 1, 1),
                    Comp::DPlusOne => (0, 0, 1, 1, 1, 1, 1),
                    Comp::APlusOne => (0, 1, 1, 0, 1, 1, 1),
                    Comp::DMinusOne => (0, 0, 0, 1, 1, 1, 0),
                    Comp::AMinusOne => (0, 1, 1, 0, 0, 1, 0),
                    Comp::DPlusA => (0, 0, 0, 0, 0, 1, 0),
                    Comp::DMinusA => (0, 0, 1, 0, 0, 1, 1),
                    Comp::AMinusD => (0, 0, 0, 0, 1, 1, 1),
                    Comp::DAndA => (0, 0, 0, 0, 0, 0, 0),
                    Comp::DOrA => (0, 0, 1, 0, 1, 0, 1),
                    Comp::M => (1, 1, 1, 0, 0, 0, 0),
                    Comp::NotM => (1, 1, 1, 0, 0, 0, 1),
                    Comp::NegM => (1, 1, 1, 0, 0, 1, 1),
                    Comp::MPlusOne => (1, 1, 1, 0, 1, 1, 1),
                    Comp::MMinusOne => (1, 1, 1, 0, 0, 1, 0),
                    Comp::DPlusM => (1, 0, 0, 0, 0, 1, 0),
                    Comp::DMinusM => (1, 0, 1, 0, 0, 1, 1),
                    Comp::MMinusD => (1, 0, 0, 0, 1, 1, 1),
                    Comp::DAndM => (1, 0, 0, 0, 0, 0, 0),
                    Comp::DOrM => (1, 0, 1, 0, 1, 0, 1),
                };

                let (j1, j2, j3) = match jump {
                    None => (0, 0, 0),
                    Some(Jump::JGT) => (0, 0, 1),
                    Some(Jump::JEQ) => (0, 1, 0),
                    Some(Jump::JGE) => (0, 1, 1),
                    Some(Jump::JLT) => (1, 0, 0),
                    Some(Jump::JNE) => (1, 0, 1),
                    Some(Jump::JLE) => (1, 1, 0),
                    Some(Jump::JMP) => (1, 1, 1),
                };

                0b1110000000000000
                    | a << 12
                    | c1 << 11
                    | c2 << 10
                    | c3 << 9
                    | c4 << 8
                    | c5 << 7
                    | c6 << 6
                    | d1 << 5
                    | d2 << 4
                    | d3 << 3
                    | j1 << 2
                    | j2 << 1
                    | j3
            }
            Instruction::Label(label) => panic!("unexpected label instruction: {}", label),
        })
        .collect()
}
