use anyhow::anyhow;
use std::io::BufRead;
use std::ops::Range;

fn bsp(range: Range<u32>, steps: &mut dyn Iterator<Item = bool>) -> anyhow::Result<Range<u32>> {
    let mut l = range.start;
    let mut h = range.end;
    while l <= h {
        let go_high = match steps.next() {
            Some(dir) => dir,
            None => return Ok(l..h),
        };
        let mid = (l + h) / 2;
        if go_high {
            l = mid;
        } else {
            h = mid;
        }
    }
    Ok(l..h)
}

fn seat(line: &str) -> anyhow::Result<u32> {
    if line.len() != 10 {
        return Err(anyhow!("Invalid line length"));
    }
    let mut row_steps = line.chars().take(7).map(|c| c == 'B');
    let mut seat_steps = line.chars().skip(7).map(|c| c == 'R');

    let row = bsp(0..128, &mut row_steps)?;
    let seat = bsp(0..8, &mut seat_steps)?;
    let id = row.start * 8 + seat.start;

    Ok(id)
}

fn part1(lines: &Vec<String>) -> u32 {
    lines.iter().map(|l| seat(l).unwrap()).max().unwrap()
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut ids: Vec<u32> = lines.iter().map(|l| seat(l).unwrap()).collect();
    ids.sort();
    let slice = ids.windows(2).find(|w| w[1] - w[0] == 2).unwrap();
    slice[0] + 1
}

fn main() -> anyhow::Result<()> {
    let lines: std::result::Result<Vec<String>, std::io::Error> =
        std::io::stdin().lock().lines().collect();
    let lines = lines?;
    let max = part1(&lines);
    let id = part2(&lines);
    println!("Part 1 max: {}", max);
    println!("Part 2 id: {}", id);
    Ok(())
}
