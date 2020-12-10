#[derive(Clone)]
enum Inst {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

use text_io::try_scan;
use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("input");

fn read_line<I: Iterator<Item=u8>>(input: &mut I) -> Result<Inst, Box<dyn std::error::Error>> {
    let inst: String;
    let arg: i64;
    try_scan!(*input => "{} {}\n", inst, arg);
    match &*inst {
        "acc" => Ok(Inst::Acc(arg)),
        "jmp" => Ok(Inst::Jmp(arg)),
        "nop" => Ok(Inst::Nop(arg)),
        _ => unreachable!(),
    }
}

fn sim(insts: &Vec<Inst>) -> Result<i64, i64> {
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut ptr = 0;

    loop {
        if !visited.insert(ptr) {
            return Err(acc);
        } else if ptr == insts.len() {
            return Ok(acc);
        }
        match &insts[ptr] {
            Inst::Acc(amt) => {
                acc += *amt;
                ptr += 1;
            },
            Inst::Jmp(amt) => {
                let newptr = (ptr as i64 + *amt) as usize;
                ptr = newptr;
            },
            Inst::Nop(_) => {
                ptr += 1;
            },
        }
    }
}

pub fn solve() {
    let mut insts = {
        let mut input = INPUT.iter().cloned();

        let mut instructions = Vec::new();

        while let Ok(inst) = read_line(&mut input) {
            instructions.push(inst);
        }

        instructions
    };

    {
        let acc = sim(&insts).unwrap_err();
        println!("Part A: {}", acc);
    }

    {
        fn swap(insts: &mut Vec<Inst>, i: usize) {
            let (amt, swap): (i64, fn(i64) -> Inst) = match &insts[i] {
                Inst::Nop(amt) => (*amt, Inst::Jmp),
                Inst::Jmp(amt) => (*amt, Inst::Nop),
                Inst::Acc(amt) => (*amt, Inst::Acc),
            };

            insts[i] = swap(amt);
        }

        for i in 0..insts.len() {
            swap(&mut insts, i);

            if let Ok(acc) = sim(&insts) {
                println!("Part B: {}", acc);
                break;
            }

            swap(&mut insts, i);
        }
    }
}
