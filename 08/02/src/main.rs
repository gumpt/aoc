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

    fn toggle(&mut self) {
        match self.kind {
            InstructionKind::Nop => self.kind = InstructionKind::Jmp,
            InstructionKind::Jmp => self.kind = InstructionKind::Nop,
            _ => {}
        }
    }
}

struct VirtualMachine {
    lines: HashMap<usize, Instruction>,
    executed: HashSet<usize>,
    iptr: usize,
    last_flipped: Option<usize>,
    acc: i64,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            lines: HashMap::new(),
            executed: HashSet::new(),
            iptr: 0,
            acc: 0,
            last_flipped: None,
        }
    }

    fn add_line(&mut self, instr: Instruction) {
        self.lines.insert(self.lines.len(), instr);
    }

    fn reset(&mut self) {
        self.iptr = 0;
        self.executed.clear();
        self.acc = 0;
    }

    fn flip_next_instruction(&mut self) {
        if let Some(v) = self.last_flipped {
            let instr = self.lines.get_mut(&v).unwrap();
            instr.toggle();
        }

        let start_index = match self.last_flipped {
            None => 0,
            Some(v) => v + 1,
        };

        if start_index >= self.lines.len() {
            panic!("ack!");
        }

        for i in start_index..self.lines.len() {
            let instr = self.lines.get_mut(&i).unwrap();
            match instr.kind {
                InstructionKind::Acc => continue,
                _ => {
                    instr.toggle();
                    self.last_flipped = Some(i);
                    return;
                }
            }
        }
    }

    fn happily_terminated(&mut self) -> bool {
        self.iptr == self.lines.len()
    }

    fn execute(&mut self) -> Option<i64> {
        if self.executed.contains(&self.iptr) || self.iptr >= self.lines.len() {
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
    while !vm.happily_terminated() {
        vm.reset();
        vm.flip_next_instruction();
        while let Some(acc) = vm.execute() {
            result = acc;
        }
    }

    println!("PART 2: {:?}", result);
}
