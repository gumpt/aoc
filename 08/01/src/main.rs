#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug)]
enum InstructionKind {
    Acc,
    Jmp,
    Nop,
}

struct Instruction {
    kind: InstructionKind,
    arg: i64,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(acc|jmp|nop)\s+(-|\+)(\d+)").unwrap();
        }

        let captures = RE.captures(line).unwrap();
        let (instr, sign, line) = (
            captures.get(1).unwrap().as_str(),
            captures.get(2).unwrap().as_str(),
            captures.get(3).unwrap().as_str(),
        );

        let kind = match instr {
            "acc" => InstructionKind::Acc,
            "jmp" => InstructionKind::Jmp,
            _ => InstructionKind::Nop,
        };

        let absolute: i64 = line.parse().unwrap();
        let arg = match sign {
            "-" => absolute * -1,
            _ => absolute,
        };

        Instruction { kind, arg }
    }
}

struct VirtualMachine {
    lines: HashMap<usize, Instruction>,
    executed: HashSet<usize>,
    iptr: usize,
    acc: i64,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            lines: HashMap::new(),
            executed: HashSet::new(),
            iptr: 0,
            acc: 0,
        }
    }

    fn add_line(&mut self, instr: Instruction) {
        self.lines.insert(self.lines.len(), instr);
    }

    fn execute(&mut self) -> Option<i64> {
        if self.executed.contains(&self.iptr) {
            return None;
        }

        self.executed.insert(self.iptr);
        let instr = &self.lines[&self.iptr];

        match instr.kind {
            InstructionKind::Acc => {
                self.acc += instr.arg;
                self.iptr += 1;
                return Some(self.acc);
            }
            InstructionKind::Jmp => {
                let uarg = instr.arg.abs() as usize;
                if instr.arg.is_positive() {
                    self.iptr += uarg;
                } else {
                    self.iptr -= uarg;
                }
                return Some(self.acc);
            }
            _ => {
                self.iptr += 1;
                return Some(self.acc);
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let mut vm = VirtualMachine::new();
    for line in stdin.lock().lines() {
        let unwrapped = line.unwrap();
        let instr = Instruction::new(&unwrapped);
        vm.add_line(instr);
    }

    let mut result: i64 = 0;
    while let Some(acc) = vm.execute() {
        result = acc;
    }

    println!("PART 1: {:?}", result);
}
