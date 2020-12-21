use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{collections::HashMap, fmt, io::BufRead};

lazy_static! {
    static ref MASK_RE: Regex = Regex::new(r"mask = ((0|1|X){36})").unwrap();
    static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

enum Op {
    Mask { or: u64, and: u64, float: u64 },
    Set { addr: u64, value: u64 },
}

impl Op {
    fn parse(s: &str) -> Result<Op> {
        let mask_caps = MASK_RE.captures(s);
        if let Some(caps) = mask_caps {
            return Op::parse_mask(&caps);
        }

        let mem_caps = MEM_RE.captures(s);
        if let Some(caps) = mem_caps {
            return Op::parse_mem(&caps);
        }

        Err(anyhow!("Invalid op: {}", s))
    }

    fn parse_mask(caps: &Captures) -> Result<Op> {
        let mut or_mask: u64 = 0;
        let mut and_mask: u64 = u64::MAX;
        let mut float_mask: u64 = 0;
        for (i, bit) in caps[1].chars().enumerate() {
            match bit {
                '0' => and_mask &= !(1u64 << (35 - i)),
                '1' => or_mask |= 1u64 << (35 - i),
                'X' => float_mask |= 1u64 << (35 - i),
                _ => return Err(anyhow!("Found invalid mask op: {}", &caps[0])),
            }
        }
        Ok(Op::Mask {
            or: or_mask,
            and: and_mask,
            float: float_mask,
        })
    }

    fn parse_mem(caps: &Captures) -> Result<Op> {
        Ok(Op::Set {
            addr: caps[1].parse::<u64>()?,
            value: caps[2].parse::<u64>()?,
        })
    }
}

fn part1(ops: &[Op]) {
    let mut or_mask = 0u64;
    let mut and_mask = u64::MAX;
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut set_mem = |addr: u64, value: u64| {
        if value == 0 {
            mem.remove(&addr);
        } else {
            mem.insert(addr, value);
        }
    };

    for op in ops {
        match op {
            Op::Mask { or, and, float: _ } => {
                or_mask = *or;
                and_mask = *and;
            }
            Op::Set { addr, value } => {
                let masked = (value & and_mask) | or_mask;
                set_mem(*addr, masked);
            }
        }
    }
    let sum: u64 = mem.values().sum();
    println!("Part1 mem sum: {}", sum);
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Address {
    address: u64,  // floating bits set to 0
    floating: u64, // bit is 1 if that part of address is floating
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..36 {
            let f_bit = (self.floating >> (35 - i)) & 1u64;
            let a_bit = (self.address >> (35 - i)) & 1u64;
            if f_bit != 0 {
                write!(f, "X")?;
            } else {
                write!(f, "{}", if a_bit != 0 { "1" } else { "0" })?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Mem {
    address: Address,
    value: i64,
}

fn intersection(a: &Address, b: &Address) -> Option<Address> {
    let float_any = a.floating | b.floating;
    if a.address & !float_any != b.address & !float_any {
        return None;
    }

    Some(Address {
        address: a.address | b.address,
        floating: a.floating & b.floating,
    })
}

fn override_mem(mem: &mut Vec<Mem>, addr: &Address) {
    let mut corrections: Vec<Mem> = Vec::new();
    let mut i = 0;
    while i < mem.len() {
        let old = &mem[i];
        match intersection(&old.address, addr) {
            Some(overlap) => {
                if overlap == old.address {
                    mem.swap_remove(i);
                } else {
                    corrections.push(Mem {
                        address: overlap,
                        value: -old.value,
                    });
                    i += 1;
                }
            }
            None => {
                i += 1;
            }
        }
    }
    mem.append(&mut corrections);
}

fn part2(ops: &[Op]) {
    let mut mem: Vec<Mem> = Vec::new();
    let mut or_mask = 0u64;
    let mut float_mask = 0u64;

    for op in ops.iter() {
        match op {
            Op::Mask { or, and: _, float } => {
                or_mask = *or;
                float_mask = *float;
            }
            Op::Set { addr, value } => {
                let masked_addr = (addr | or_mask) & !float_mask;
                let new_addr = Address {
                    address: masked_addr,
                    floating: float_mask,
                };
                override_mem(&mut mem, &new_addr);
                mem.push(Mem {
                    address: new_addr,
                    value: *value as i64,
                });
            }
        }
    }

    let sum: i64 = mem
        .iter()
        .map(|m| m.value * 2i64.pow(m.address.floating.count_ones() as u32))
        .sum();
    println!("Part2 mem sum: {}", sum);
}

fn main() -> Result<()> {
    let ops: Result<Vec<Op>> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| Op::parse(line?.as_str()))
        .collect();
    let ops = ops?;

    part1(&ops);
    part2(&ops);

    Ok(())
}
