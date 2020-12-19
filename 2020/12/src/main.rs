use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"(N|S|E|W|F|L|R)(\d+)").unwrap();
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    F(i32),
    Rotate(Rotation),
}

fn rotation_from_degrees(deg: i32) -> Result<Rotation> {
    match deg {
        90 => Ok(Rotation::Rotate90),
        180 => Ok(Rotation::Rotate180),
        270 => Ok(Rotation::Rotate270),
        _ => Err(anyhow!("Invalid rotation degree: {}", deg)),
    }
}

impl Instruction {
    fn parse(s: &str) -> Result<Instruction> {
        let err = || anyhow!("Invalid instuction: {}", s);
        let caps = RE_INSTRUCTION.captures(s).ok_or_else(err)?;
        let nb: i32 = caps[2].parse()?;
        match &(caps[1]) {
            "N" => Ok(Instruction::N(nb)),
            "S" => Ok(Instruction::S(nb)),
            "E" => Ok(Instruction::E(nb)),
            "W" => Ok(Instruction::W(nb)),
            "F" => Ok(Instruction::F(nb)),
            "L" => Ok(Instruction::Rotate(rotation_from_degrees(nb)?)),
            "R" => Ok(Instruction::Rotate(rotation_from_degrees(360 - nb)?)),
            _ => Err(err()),
        }
    }
}

fn rotate(p: &Point, rotation: Rotation) -> Point {
    match rotation {
        Rotation::Rotate90 => Point { x: -p.y, y: p.x },
        Rotation::Rotate180 => Point { x: -p.x, y: -p.y },
        Rotation::Rotate270 => Point { x: p.y, y: -p.x },
    }
}

fn run_one(op: Instruction, pos: &mut Point, dir: &mut Point) {
    match op {
        Instruction::N(delta) => pos.y += delta,
        Instruction::S(delta) => pos.y -= delta,
        Instruction::E(delta) => pos.x += delta,
        Instruction::W(delta) => pos.x -= delta,
        Instruction::F(delta) => {
            pos.x += dir.x * delta;
            pos.y += dir.y * delta;
        }
        Instruction::Rotate(rot) => *dir = rotate(dir, rot),
    };
}

fn run_many(instructions: &[Instruction]) -> Point {
    let mut pos = Point { x: 0, y: 0 };
    let mut dir = Point { x: 1, y: 0 };
    instructions.iter().for_each(|instr| {
        run_one(*instr, &mut pos, &mut dir);
    });
    pos
}

fn run_one_wp(op: Instruction, pos: &mut Point, wp: &mut Point) {
    match op {
        Instruction::N(delta) => wp.y += delta,
        Instruction::S(delta) => wp.y -= delta,
        Instruction::E(delta) => wp.x += delta,
        Instruction::W(delta) => wp.x -= delta,
        Instruction::F(delta) => {
            pos.x += wp.x * delta;
            pos.y += wp.y * delta;
        }
        Instruction::Rotate(rot) => *wp = rotate(wp, rot),
    };
}

fn run_many_wp(instructions: &[Instruction]) -> Point {
    let mut pos = Point { x: 0, y: 0 };
    let mut wp = Point { x: 10, y: 1 };
    instructions.iter().for_each(|instr| {
        run_one_wp(*instr, &mut pos, &mut wp);
    });
    pos
}

fn main() -> Result<()> {
    let instructions: Result<Vec<Instruction>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| Instruction::parse(&line?))
        .collect();
    let instructions = instructions?;

    let pos1 = run_many(instructions.as_slice());
    println!(
        "Manhattan distance after instructions: {}",
        pos1.x.abs() + pos1.y.abs()
    );

    let pos2 = run_many_wp(instructions.as_slice());
    println!(
        "Manhattan distance after waypoint instructions: {}",
        pos2.x.abs() + pos2.y.abs()
    );

    Ok(())
}
