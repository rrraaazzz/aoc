use std::{collections::HashSet, default::Default, io::BufRead};

fn build_groups<G, F>(
    lines: impl Iterator<Item = String>,
    mut grow_group: F,
) -> impl Iterator<Item = G>
where
    F: FnMut(&mut G, &str),
    G: Default,
{
    // Make sure there's an empty line at the end
    let chained = lines.chain(std::iter::once(String::new()));
    let mut current_group: G = Default::default();
    chained.filter_map(move |line| {
        if line.len() == 0 {
            let mut result = Default::default();
            std::mem::swap(&mut result, &mut current_group);
            return Some(result);
        }
        grow_group(&mut current_group, &line);
        None
    })
}

fn part1(lines: impl Iterator<Item = String>) -> anyhow::Result<usize> {
    let groups = build_groups(lines, |group: &mut HashSet<char>, line: &str| {
        group.extend(line.chars());
    });
    Ok(groups.map(|set| set.len()).sum())
}

#[derive(Default)]
struct Group {
    answers: HashSet<char>,
    is_initialized: bool,
}

fn part2(lines: impl Iterator<Item = String>) -> anyhow::Result<usize> {
    let groups = build_groups(lines, |group: &mut Group, line: &str| {
        if group.is_initialized {
            let person_answers: HashSet<char> = line.chars().collect();
            let mut intersection: HashSet<char> =
                group.answers.intersection(&person_answers).map(|c| c.clone()).collect();
            std::mem::swap(&mut group.answers, &mut intersection);
        } else {
            group.answers.extend(line.chars());
            group.is_initialized = true;
        }
    });
    Ok(groups.map(|g| g.answers.len()).sum())
}

fn main() -> anyhow::Result<()> {
    let input = std::io::stdin();
    let lines = input.lock().lines().map(Result::unwrap);
    if std::env::args().any(|arg| arg == "--part2") {
        println!("Count: {}", part2(lines)?);
    } else {
        println!("Count: {}", part1(lines)?);
    }
    Ok(())
}