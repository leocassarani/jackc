use std::fmt;
use std::str::FromStr;

#[macro_export]
macro_rules! asm {
    (@$label:expr) => {
        $crate::asm::Instruction::A($crate::asm::Load::from($label))
    };
    ($dest:ident = $comp:expr) => {
        $crate::asm::Instruction::C(
            Some($crate::asm::Dest::$dest),
            stringify!($comp).parse().unwrap(),
            None,
        )
    };
    ($comp:expr ; $jump:ident) => {
        $crate::asm::Instruction::C(
            None,
            stringify!($comp).parse().unwrap(),
            Some($crate::asm::Jump::$jump),
        )
    };
    (($label:expr)) => {
        $crate::asm::Instruction::Label(String::from($label))
    };
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    A(Load),
    C(Option<Dest>, Comp, Option<Jump>),
    Label(String),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::A(load) => write!(f, "@{}", load),
            Instruction::C(Some(dest), comp, Some(jump)) => write!(f, "{}={};{}", dest, comp, jump),
            Instruction::C(Some(dest), comp, None) => write!(f, "{}={}", dest, comp),
            Instruction::C(None, comp, Some(jump)) => write!(f, "{};{}", comp, jump),
            Instruction::C(None, comp, None) => write!(f, "{}", comp),
            Instruction::Label(s) => write!(f, "({})", s),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Load {
    Constant(u16),
    Symbol(String),
}

impl From<u16> for Load {
    fn from(n: u16) -> Self {
        Load::Constant(n)
    }
}

impl From<&str> for Load {
    fn from(sym: &str) -> Self {
        Load::Symbol(sym.to_owned())
    }
}

impl From<String> for Load {
    fn from(sym: String) -> Self {
        Load::Symbol(sym)
    }
}

impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Load::Constant(n) => write!(f, "{}", n),
            Load::Symbol(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Dest {
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

impl fmt::Display for Dest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dest::M => write!(f, "M"),
            Dest::D => write!(f, "D"),
            Dest::MD => write!(f, "MD"),
            Dest::A => write!(f, "A"),
            Dest::AM => write!(f, "AM"),
            Dest::AD => write!(f, "AD"),
            Dest::AMD => write!(f, "AMD"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Comp {
    Zero,
    One,
    NegOne,
    D,
    A,
    NotD,
    NotA,
    NegD,
    NegA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    NegM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

impl fmt::Display for Comp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comp::Zero => write!(f, "0"),
            Comp::One => write!(f, "1"),
            Comp::NegOne => write!(f, "-1"),
            Comp::D => write!(f, "D"),
            Comp::A => write!(f, "A"),
            Comp::NotD => write!(f, "!D"),
            Comp::NotA => write!(f, "!A"),
            Comp::NegD => write!(f, "-D"),
            Comp::NegA => write!(f, "-A"),
            Comp::DPlusOne => write!(f, "D+1"),
            Comp::APlusOne => write!(f, "A+1"),
            Comp::DMinusOne => write!(f, "D-1"),
            Comp::AMinusOne => write!(f, "A-1"),
            Comp::DPlusA => write!(f, "D+A"),
            Comp::DMinusA => write!(f, "D-A"),
            Comp::AMinusD => write!(f, "A-D"),
            Comp::DAndA => write!(f, "D&A"),
            Comp::DOrA => write!(f, "D|A"),
            Comp::M => write!(f, "M"),
            Comp::NotM => write!(f, "!M"),
            Comp::NegM => write!(f, "-M"),
            Comp::MPlusOne => write!(f, "M+1"),
            Comp::MMinusOne => write!(f, "M-1"),
            Comp::DPlusM => write!(f, "D+M"),
            Comp::DMinusM => write!(f, "D-M"),
            Comp::MMinusD => write!(f, "M-D"),
            Comp::DAndM => write!(f, "D&M"),
            Comp::DOrM => write!(f, "D|M"),
        }
    }
}

impl FromStr for Comp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.replace(' ', "")[..] {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::NegOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "-D" => Ok(Comp::NegD),
            "-A" => Ok(Comp::NegA),
            "D+1" => Ok(Comp::DPlusOne),
            "A+1" => Ok(Comp::APlusOne),
            "D-1" => Ok(Comp::DMinusOne),
            "A-1" => Ok(Comp::AMinusOne),
            "D+A" => Ok(Comp::DPlusA),
            "D-A" => Ok(Comp::DMinusA),
            "A-D" => Ok(Comp::AMinusD),
            "D&A" => Ok(Comp::DAndA),
            "D|A" => Ok(Comp::DOrA),
            "M" => Ok(Comp::M),
            "!M" => Ok(Comp::NotM),
            "-M" => Ok(Comp::NegM),
            "M+1" => Ok(Comp::MPlusOne),
            "M-1" => Ok(Comp::MMinusOne),
            "D+M" => Ok(Comp::DPlusM),
            "D-M" => Ok(Comp::DMinusM),
            "M-D" => Ok(Comp::MMinusD),
            "D&M" => Ok(Comp::DAndM),
            "D|M" => Ok(Comp::DOrM),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Jump {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl fmt::Display for Jump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Jump::JGT => write!(f, "JGT"),
            Jump::JEQ => write!(f, "JEQ"),
            Jump::JGE => write!(f, "JGE"),
            Jump::JLT => write!(f, "JLT"),
            Jump::JNE => write!(f, "JNE"),
            Jump::JLE => write!(f, "JLE"),
            Jump::JMP => write!(f, "JMP"),
        }
    }
}
