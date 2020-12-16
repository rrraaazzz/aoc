use std::{collections::HashSet, ops::Range};
use std::io::BufRead;
use anyhow::{Result, anyhow};

const WINDOW_SIZE: usize = 25;

fn is_valid_nb(nb: i64, prev: &[i64], prev_set: &HashSet<i64>) -> bool {
    for p in prev {
        if prev_set.contains(&(nb - p)) {
            return true;
        }
    }
    return false;
}

fn find_invalid_nb(numbers: &Vec<i64>) -> Result<i64> {
    let mut window: HashSet<i64> = HashSet::new();
    numbers.iter().take(WINDOW_SIZE).for_each(|n| {
        window.insert(*n);
    });
    for i in WINDOW_SIZE..numbers.len() {
        let nb = numbers[i];
        if !is_valid_nb(nb, &numbers[(i - WINDOW_SIZE) .. i], &window) {
            return Ok(nb);
        }
        window.remove(&numbers[i - WINDOW_SIZE]);
        window.insert(nb);
    }
    Err(anyhow!("Couldn't find any invalid numbers"))
}

fn find_sum_range(numbers: &Vec<i64>, target: i64) -> Result<Range<usize>> {
    let mut begin = 0;
    let mut end = 0;
    let mut sum = 0;
    while begin < numbers.len() {
        if sum == target {
            return Ok(begin..end);
        }
        while sum < target && end < numbers.len() {
            sum += numbers[end];
            end += 1;
        }
        while sum > target && begin < end {
            sum -= numbers[begin];
            begin += 1;
        }
    }
    Err(anyhow!("Couldn't find a valid range"))
}

fn main() -> Result<()> {
    let numbers: Vec<i64> = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    let invalid = find_invalid_nb(&numbers)?;
    println!("Invalid number is: {}", invalid);

    let range = find_sum_range(&numbers, invalid)?;
    let min = numbers[range.clone()].iter().min().unwrap();
    let max = numbers[range.clone()].iter().max().unwrap();
    println!("Range is {:?}, min is {}, max is {}, sum is {}", &range, min, max, min + max);

    Ok(())
}
