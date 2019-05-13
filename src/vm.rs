use instruction::Opcode;
#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new () -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }
    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            },
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize; // casting to use an index
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32; // cast to i32 as that is the size of our registers
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            },
            Opcode::EQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                if value1 == value2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            },
            Opcode::JMPE => {
                let register = self.next_8_bits() as usize;
                let value = self.registers[register] as usize;
                if self.equal_flag {
                    self.pc = value;
                }
            },
            Opcode::JNEQ => {
                let register = self.next_8_bits() as usize;
                let value = self.registers[register] as usize;
                if !self.equal_flag {
                    self.pc = value;
                }
            },
            Opcode::NEQ => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                if value1 != value2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            },
            Opcode::GT => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                if value1 > value2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
            },
            Opcode::LT => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                if value1 < value2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
            },
            Opcode::GTE => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                if value1 >= value2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
            },
            Opcode::LTE => {
                let value1 = self.registers[self.next_8_bits() as usize];
                let value2 = self.registers[self.next_8_bits() as usize];
                if value1 <= value2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
            },
            _ => {
                println!("Unknown opcode found");
                return false;
            }
        }
        return true;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        // bit manipulations to grab the next 16 bits of the program
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2; // have to adjust by 2 as we are grabbing 16 bits instead of 
        return result;
    }   

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm () {
        let test_vm = VM::new();
        for x in 0..32 {
             assert_eq!(test_vm.registers[x], 0)
        }
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![254,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![0,0,1,244]; // representing 500 in u8s little endian
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }
    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![1,0,1,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2],15);
    }
    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![2,1,0,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }
    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![3,1,0,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 50);
    }
    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![4,1,0,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
        assert_eq!(test_vm.remainder, 0);
    }
    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6,0,0,0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![7, 0, 0, 0, 4, 1, 0, 2, 4, 1, 0, 2];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![8, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }
    
    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.registers[1] = 2;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 3;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
    #[test]
    fn test_jmpe_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.equal_flag = true;
        test_vm.program = vec![10, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }
    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.equal_flag = false;
        test_vm.program = vec![11, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }
    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 5;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run();
        assert_eq!(test_vm.equal_flag, false);
    }
}