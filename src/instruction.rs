#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPB,
    JMPF,
    EQ,
    JEQ,
    JNEQ,
    JMPE,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    IGL,
}
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}
impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            5 => return Opcode::HLT,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPB,
            8 => return Opcode::JMPF,
            9 => return Opcode::EQ,
            10 => return Opcode::JMPE,
            11 => return Opcode::JNEQ,
            12 => return Opcode::NEQ,
            13 => return Opcode::GT,
            14 => return Opcode::LT,
            15 => return Opcode::GTE,
            16 => return Opcode::LTE,
            _ => return Opcode::IGL
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}