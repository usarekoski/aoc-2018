use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum OpCode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}
use self::OpCode::*;

#[derive(Debug)]
pub struct Instruction {
    opcode: OpCode,
    a: i64,
    b: i64,
    c: i64,
}

type Registers = [i64; 6];

fn parse_ins(s: &str) -> Instruction {
    let mut v = s.split(' ');
    let opcode = match v.next().unwrap() {
        "addr" => addr,
        "addi" => addi,
        "mulr" => mulr,
        "muli" => muli,
        "banr" => banr,
        "bani" => bani,
        "borr" => borr,
        "bori" => bori,
        "setr" => setr,
        "seti" => seti,
        "gtir" => gtir,
        "gtri" => gtri,
        "gtrr" => gtrr,
        "eqir" => eqir,
        "eqri" => eqri,
        "eqrr" => eqrr,
        c => panic!("Unknown instruction: {}", c),
    };
    let a = v.next().unwrap().parse().unwrap();
    let b = v.next().unwrap().parse().unwrap();
    let c = v.next().unwrap().parse().unwrap();

    Instruction { opcode, a, b, c }
}

pub struct VM {
    ip: i64,
    bound_reg: usize,
    registers: Registers,
    pub program: Vec<Instruction>,
}

impl FromStr for VM {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bound_reg = s
            .trim()
            .lines()
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let program: Vec<Instruction> = s
            .trim()
            .lines()
            .skip(1)
            .map(|l| parse_ins(l.trim()))
            .collect();

        Ok(VM {
            ip: 0,
            bound_reg: bound_reg,
            program: program,
            registers: [0; 6],
        })
    }
}

impl VM {
    pub fn run(&mut self) -> bool {
        let r = &mut self.registers;
        let ins = {
            let idx = if self.ip < 0 || self.ip >= self.program.len() as i64 {
                return false;
            } else {
                self.ip as usize
            };
            &self.program[idx]
        };
        let a = ins.a as usize;
        let b = ins.b as usize;
        let c = ins.c as usize;
        r[self.bound_reg] = self.ip as i64;

        match ins.opcode {
            addr => r[c] = r[a] + r[b],
            addi => r[c] = r[a] + ins.b,
            mulr => r[c] = r[a] * r[b],
            muli => r[c] = r[a] * ins.b,
            banr => r[c] = r[a] & r[b],
            bani => r[c] = r[a] & ins.b,
            borr => r[c] = r[a] | r[b],
            bori => r[c] = r[a] | ins.b,
            setr => r[c] = r[a],
            seti => r[c] = ins.a,
            gtir => r[c] = if ins.a > r[b] { 1 } else { 0 },
            gtri => r[c] = if r[a] > ins.b { 1 } else { 0 },
            gtrr => r[c] = if r[a] > r[b] { 1 } else { 0 },
            eqir => r[c] = if ins.a == r[b] { 1 } else { 0 },
            eqri => r[c] = if r[a] == ins.b { 1 } else { 0 },
            eqrr => r[c] = if r[a] == r[b] { 1 } else { 0 },
        }

        self.ip = r[self.bound_reg];
        self.ip += 1;

        true
    }

    // Print instructions as they were using variables.
    pub fn decompile(ins: &Instruction) -> String {
        let r = ["a", "b", "c", "d", "e", "ip"];
        let a = ins.a as usize;
        let b = ins.b as usize;
        let c = ins.c as usize;

        match ins.opcode {
            addr => format!("{} = {} + {}", r[c], r[a], r[b]),
            addi => format!("{} = {} + {}", r[c], r[a], ins.b),
            mulr => format!("{} = {} * {}", r[c], r[a], r[b]),
            muli => format!("{} = {} * {}", r[c], r[a], ins.b),
            banr => format!("{} = {} & {}", r[c], r[a], r[b]),
            bani => format!("{} = {} & {}", r[c], r[a], ins.b),
            borr => format!("{} = {} | {}", r[c], r[a], r[b]),
            bori => format!("{} = {} | {}", r[c], r[a], ins.b),
            setr => format!("{} = {}", r[c], r[a]),
            seti => format!("{} = {}", r[c], ins.a),
            gtir => format!("{} = if {} > {} {{ 1 }} else {{ 0 }}", r[c], ins.a, r[b]),
            gtri => format!("{} = if {} > {} {{ 1 }} else {{ 0 }}", r[c], r[a], ins.b),
            gtrr => format!("{} = if {} > {} {{ 1 }} else {{ 0 }}", r[c], r[a], r[b]),
            eqir => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r[c], ins.a, r[b]),
            eqri => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r[c], r[a], ins.b),
            eqrr => format!("{} = if {} == {} {{ 1 }} else {{ 0 }}", r[c], r[a], r[b]),
        }
    }
}

pub fn solve1(s: String) -> i64 {
    let mut vm: VM = s.parse().unwrap();
    for (i, line) in vm.program.iter().enumerate() {
        println!("{} {}", i, VM::decompile(line));
    }

    let r2 = loop {
        let no_halt = vm.run();
        assert_eq!(no_halt, true);
        if vm.ip == 28 {
            break vm.registers[2];
        }
    };

    r2
}

pub fn solve2(s: String) -> i64 {
    let mut vm: VM = s.parse().unwrap();
    let mut r2s: Vec<i64> = vec![];

    let max_r2 = loop {
        let no_halt = vm.run();
        assert_eq!(no_halt, true);
        if vm.ip == 28 {
            let r2 = vm.registers[2];
            if r2s.iter().any(|&r| r == r2) {
                // Last before looping.
                break *r2s.last().unwrap();
            }
            r2s.push(r2);
        }
    };

    max_r2
}
