// TODO: Evaluate if it would be better to find a more advanced enumerables
// implementation in a create. since rust enums don't allow for metadata.

#[derive(Debug)]
enum Instruction {
    MOV(u8, u8),     // MOV VALUE REGISTER
    STATE(),         // STATE (print state of VM)
    ADD(u8, u8, u8), // ADD REG1 REG2 OUTREG
}

impl Instruction {
    fn from(b: &[u8]) -> Option<Self> {
        Some(match b.get(0)? {
            1 => Instruction::MOV(*b.get(1)?, *b.get(2)?),
            2 => Instruction::STATE(),
            3 => Instruction::ADD(*b.get(1)?, *b.get(2)?, *b.get(3)?),
            _ => return None,
        })
    }

    fn args(&self) -> u8 {
        match self {
            Instruction::MOV(_, _) => 2,
            Instruction::STATE() => 0,
            Instruction::ADD(_, _, _) => 3,
        }
    }
}

// So i can change the datatype the assembly language uses easily.
type dt = u8;

#[derive(Debug)]
struct Registers {
    // TODO(BUG): program cannot be larger then u8 in size. because we limit
    // pc to u8
    pc: dt,

    // TODO: larger registers? would require rethinking the binary format.
    g1: dt,
    g2: dt,
    g3: dt,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            pc: 0,
            g1: 0,
            g2: 0,
            g3: 0,
        }
    }

    fn get(&self, i: dt) -> Option<dt> {
        Some(match i {
            0 => self.pc,
            1 => self.g1,
            2 => self.g2,
            3 => self.g3,
            _ => return None,
        })
    }

    fn set(&mut self, i: dt, v: dt) {
        match i {
            0 => self.pc = v,
            1 => self.g1 = v,
            2 => self.g2 = v,
            3 => self.g3 = v,
            _ => panic!("Unknown register number {}", i),
        }
    }
}

struct Bytecode(Vec<Instruction>);

impl Bytecode {
    //                        TODO: Use result for better errors/debugging
    fn from(raw_bytecode: &[u8]) -> Option<Bytecode> {
        let mut i = 0;
        let mut bytecode = Vec::new();

        while i < raw_bytecode.len() {
            let ins = Instruction::from(&raw_bytecode[i..])?;
            i += ins.args() as usize;
            i += 1;
            bytecode.push(ins);
        }

        Some(Bytecode(bytecode))
    }
}

// A VM interpreting some code.
struct VM {
    registers: Registers,
}

impl VM {
    fn new() -> VM {
        VM {
            registers: Registers::new(),
        }
    }

    fn run(&mut self, bc: &Bytecode) {
        while (self.registers.pc as usize) < bc.0.len() {
            let ins = &bc.0[self.registers.pc as usize];
            self.registers.pc += 1;

            eprintln!("{:?}", ins);
            std::thread::sleep_ms(2000);

            match ins {
                Instruction::MOV(val, reg) => {
                    self.registers.set(*reg, *val);
                }
                Instruction::STATE() => {
                    eprintln!("===== Registers =====");
                    eprintln!("{:?}", self.registers);
                }
                Instruction::ADD(r1, r2, r3) => {
                    self.registers.set(
                        *r3,
                        self.registers.get(*r1).unwrap() + self.registers.get(*r2).unwrap(),
                    );
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    use std::fs;

    let bytes = fs::read("compiled.uo")?;

    let bytecode = Bytecode::from(&bytes).unwrap();
    let mut vm = VM::new();
    vm.run(&bytecode);

    Ok(())
}
