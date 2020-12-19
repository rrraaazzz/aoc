use anyhow::Result;
use std::collections::HashMap;
use num_bigint::BigUint;

struct Congruence {
    value: i64,
    modulus: i64,
}

fn part1(arrival_time: i64, bus_times: &[Congruence]) {
    let (bus_id, wait_time) = bus_times
        .iter()
        .map(|c| c.modulus)
        .map(|t| (t, t - arrival_time % t))
        .min_by_key(|(_, wait_time)| wait_time.clone())
        .unwrap();
    println!("Part1: {}", bus_id * wait_time);
}

// Computes gcd(a, b) and also coefficients x, y for
// which a*x + b*y == d.
//
// See: https://cp-algorithms.com/algebra/extended-euclid-algorithm.html
fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, x1, y1) = extended_euclid(b, a % b);
    (d, y1, x1 - y1 * (a / b))
}

// Finds a solution to the chinese remainder theorem using garner's
// algorithm.
//
// See: https://cp-algorithms.com/algebra/chinese-remainder-theorem.html
fn garner(congruences: &[Congruence]) -> BigUint {
    let mut r: HashMap<(usize, usize), i64> = HashMap::new();
    let len = congruences.len();
    for i in 0..len {
        for j in 0..i {
            let p_i = congruences[i].modulus;
            let p_j = congruences[j].modulus;
            let (_, r_j_i, _) = extended_euclid(p_j, p_i);
            r.insert((j, i), r_j_i);
        }
    }

    let mut x: Vec<i64> = Vec::new();
    x.resize_with(len, Default::default);

    for i in 0..len {
        x[i] = congruences[i].value;
        for j in 0..i {
            x[i] = r.get(&(j, i)).unwrap() * (x[i] - x[j]);
            x[i] = x[i] % congruences[i].modulus;
            if x[i] < 0 {
                x[i] += congruences[i].modulus;
            }
        }
    }

    let mut result: BigUint = BigUint::default();
    for i in 0..len {
        let mut term: BigUint = BigUint::from(x[i] as u64);
        for j in 0..i {
            term *= congruences[j].modulus as u64;
        }
        result += term;
    }
    result
}



fn main() -> Result<()> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let arrival_time: i64 = line.trim().parse()?;

    let mut congruences: Vec<Congruence> = Vec::new();
    
    line.clear();
    std::io::stdin().read_line(&mut line)?;

    for (index, term) in line.trim().split(',').enumerate() {
        if term == "x" {
            continue;
        }
        let term = term.parse::<i64>().unwrap();
        congruences.push(Congruence {
            value: (term - index as i64) % term,
            modulus: term,
        });
    }

    part1(arrival_time, &congruences);

    println!("Part 2: {}", garner(&congruences));
    Ok(())
}
