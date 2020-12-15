use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

lazy_static! {
    static ref OP_RE: Regex = Regex::new(r"(acc|jmp|nop) ((\+|-)\d+)?").unwrap();
}

#[derive(Copy, Clone)]
enum Op {
    Acc(i32),
    Nop(i32),
    Jmp(i32),
}

fn parse_line(line: &str) -> Result<Op> {
    let err = || anyhow!("Invalid input line: {}", line);
    let caps = OP_RE.captures(line).ok_or(err())?;
    let arg = caps[2].parse::<i32>()?;
    match &caps[1] {
        "acc" => Ok(Op::Acc(arg)),
        "nop" => Ok(Op::Nop(arg)),
        "jmp" => Ok(Op::Jmp(arg)),
        &_ => Err(err()),
    }
}

fn run_op(op: &Op, ip: &mut i32, acc: &mut i32) {
    match op {
        Op::Acc(delta) => {
            *acc += delta;
            *ip += 1
        }
        Op::Nop(_) => *ip += 1,
        Op::Jmp(delta) => *ip += delta,
    }
}

fn last_acc_before_loop_or_end(ops: &Vec<Op>) -> i32 {
    let mut acc = 0;
    let mut ip = 0;
    let mut executed: HashSet<i32> = HashSet::new();
    loop {
        if ip >= ops.len() as i32 {
            return acc;
        }
        if executed.insert(ip) {
            let op = &ops[ip as usize];
            run_op(op, &mut ip, &mut acc);
        } else {
            return acc;
        }
    }
}

fn jump_sources(ops: &Vec<Op>) -> HashMap<i32, Vec<i32>> {
    let mut result = HashMap::new();
    ops.iter().enumerate().for_each(|(i, op)| {
        if let Op::Jmp(delta) = op {
            let source = i as i32;
            let target: i32 = source + delta;
            result
                .entry(target)
                .or_insert_with(|| Vec::new())
                .push(source);
        }
    });
    result
}

fn is_jmp(op: &Op) -> bool {
    match op {
        Op::Jmp(_) => true,
        _ => false,
    }
}

// This is basically a DFS in a graph that has instruction pointers as nodes. The
// edges link instructions that can follow each other in the program flow.
//
// It returns the set of instruction pointers from which the program can halt.
fn endings(ops: &Vec<Op>, jump_sources: &HashMap<i32, Vec<i32>>) -> HashSet<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut leads: Vec<i32> = Vec::new();
    leads.push(ops.len() as i32);
    while let Some(target) = leads.pop() {
        visited.insert(target);

        // If the previous op is a nop or acc, it's an implicit source for the
        // current location.
        if target > 0 && !is_jmp(&ops[target as usize - 1]) {
            leads.push(target - 1);
        }

        // Add the jumps that lead to this target.
        jump_sources.get(&target).map(|sources| {
            leads.extend(sources.iter());
        });
    }
    visited
}

fn flip_op(op: &Op) -> Op {
    match op {
        Op::Jmp(d) => Op::Nop(*d),
        Op::Nop(d) => Op::Jmp(*d),
        Op::Acc(d) => Op::Acc(*d),
    }
}

fn alternate_ip_offset(op: &Op) -> i32 {
    match op {
        Op::Jmp(_) => 1,
        Op::Nop(delta) => *delta,
        Op::Acc(_) => 1,
    }
}

// Finds the first instruction that when flipped from nop to jmp or jmp to nop
// will reach one of the valid endings (see `fn endings`). The instructions
// reachable from the start are guaranteed to loop forever, so none of the
// valid endings can include any of these instructions. Thus even after flipping
// one of them the endings remain valid.
fn find_corrupted_op(ops: &Vec<Op>) -> i32 {
    let jump_sources = jump_sources(ops);
    let endings = endings(ops, &jump_sources);
    let mut acc = 0;
    let mut ip = 0;
    loop {
        let op = &ops[ip as usize];
        let alt_target = ip + alternate_ip_offset(op);
        if endings.contains(&alt_target) {
            return ip;
        }
        run_op(op, &mut ip, &mut acc);
    }
}

fn main() -> Result<()> {
    let ops: Result<Vec<Op>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_line(&l?))
        .collect();
    let mut ops = ops?;
    println!(
        "Last acc before loop: {}",
        last_acc_before_loop_or_end(&ops)
    );

    let corrupted_ip = find_corrupted_op(&ops) as usize;
    ops[corrupted_ip] = flip_op(&ops[corrupted_ip]);
    println!(
        "Acc after fixing corruption: {}",
        last_acc_before_loop_or_end(&ops)
    );

    Ok(())
}
