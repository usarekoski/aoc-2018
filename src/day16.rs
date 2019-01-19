use std::collections::{HashMap, HashSet};
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
const ALL_OPCODES: &[OpCode] = &[
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

struct Instruction {
    opcode: OpCode,
    a: i64,
    b: i64,
    c: i64,
}

struct UnknownInstruction {
    opcode: i64,
    a: i64,
    b: i64,
    c: i64,
}

type Registers = [i64; 4];

struct Sample {
    registers_before: Registers,
    instruction: UnknownInstruction,
    registers_after: Registers,
}

fn fill_slice<I: IntoIterator<Item = i64>>(iter: I) -> [i64; 4] {
    let mut r1 = [0; 4];
    for (ret, src) in r1.iter_mut().zip(iter) {
        *ret = src;
    }
    r1
}

impl FromStr for Sample {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_reg = |line| {
            let prefix = "Before: [".len()..;
            s.lines()
                .nth(line)
                .unwrap()
                .trim()
                .get(prefix.clone())
                .unwrap()
                .trim_end_matches(']')
                .split(',')
                .map(|d| d.trim().parse().unwrap())
        };
        let ins: Vec<i64> = s
            .lines()
            .nth(1)
            .unwrap()
            .trim()
            .split(' ')
            .map(|d| d.trim().parse().unwrap())
            .collect();

        Ok(Sample {
            registers_before: fill_slice(parse_reg(0)),
            instruction: UnknownInstruction {
                opcode: ins[0],
                a: ins[1],
                b: ins[2],
                c: ins[3],
            },
            registers_after: fill_slice(parse_reg(2)),
        })
    }
}

fn run(r: &mut Registers, ins: &Instruction) {
    let a = ins.a as usize;
    let b = ins.b as usize;
    let c = ins.c as usize;

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
}

// Test if sample works with given opcode.
fn test_sample(op: OpCode, sample: &Sample) -> bool {
    let mut r: Registers = sample.registers_before.clone();
    let ins = Instruction {
        opcode: op,
        a: sample.instruction.a,
        b: sample.instruction.b,
        c: sample.instruction.c,
    };
    run(&mut r, &ins);

    r == sample.registers_after
}

pub fn solve1(s: String) -> i64 {
    let samples_end = s.find("\n\n\n").unwrap();
    let (samples_s, _) = s.split_at(samples_end);
    let samples: Vec<Sample> = samples_s
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut count = 0;
    for sample in samples.iter() {
        let matching: i64 = ALL_OPCODES
            .iter()
            .map(|op| if test_sample(*op, sample) { 1 } else { 0 })
            .sum();
        if matching >= 3 {
            count += 1;
        }
    }

    count
}

pub fn solve2(s: String) -> i64 {
    let samples_end = s.find("\n\n\n").unwrap();
    let (samples_s, program_s) = s.split_at(samples_end);
    let samples: Vec<Sample> = samples_s
        .split("\n\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut possible_opcodes: HashMap<i64, HashSet<OpCode>> = HashMap::new();
    for sample in samples.iter() {
        let opcode = sample.instruction.opcode;
        let matching: HashSet<OpCode> = ALL_OPCODES
            .iter()
            .map(|&op| {
                if test_sample(op, sample) {
                    Some(op)
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        let e = possible_opcodes.entry(opcode).or_insert(matching.clone());
        let diff: Vec<_> = e.difference(&matching).map(|v| *v).collect();
        for d in &diff {
            e.remove(&d);
        }
    }

    println!("{:?}", possible_opcodes);

    let mut mapping: HashMap<i64, OpCode> = HashMap::new();
    while possible_opcodes.len() > 0 {
        let found: Vec<_> = possible_opcodes
            .iter()
            .filter(|(_op, matching)| matching.len() == 1)
            .map(|(op, matching)| (*op, *matching.iter().nth(0).unwrap()))
            .collect();
        for (op, matching) in found.iter() {
            possible_opcodes.remove(op);
            mapping.insert(*op, *matching);
            for (_, values) in possible_opcodes.iter_mut() {
                values.remove(matching);
            }
        }
    }

    println!("{:?}", mapping);

    let program: Vec<Instruction> = program_s
        .trim()
        .lines()
        .map(|l| {
            let input: Vec<i64> = l
                .trim()
                .split(' ')
                .map(|d| d.trim().parse().unwrap())
                .collect();
            Instruction {
                opcode: *mapping.get(&input[0]).unwrap(),
                a: input[1],
                b: input[2],
                c: input[3],
            }
        })
        .collect();

    let mut registers: Registers = [0; 4];
    for ins in &program {
        run(&mut registers, ins);
    }

    registers[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]


";

        assert_eq!(solve1(input.to_string()), 1);
    }
}
