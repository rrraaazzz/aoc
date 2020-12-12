use std::io::BufRead;
use anyhow::anyhow;

struct Map {
    trees: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn parse(iter: impl Iterator<Item=std::io::Result<String>>) -> anyhow::Result<Map> {
        let mut result = Map { trees: Vec::new(), width: 0, height: 0 };
        for line in iter {
            result.add_line(&line?)?;
        }
        Ok(result)
    }

    pub fn add_line(&mut self, line: &str) -> anyhow::Result<()> {
        if self.width == 0 {
            self.width = line.len();
        } else if self.width != line.len() {
            return Err(anyhow!("Expecting all input lines to have the same length"));
        }

        self.height += 1;
        self.trees.extend(line.chars().map(|x| x == '#'));

        Ok(())
    }

    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        if y >= self.height {
            return false;
        }
        let real_x = x % self.width;
        let index = y * self.width + real_x;
        self.trees[index]
    }

    pub fn is_end(&self, y: usize) -> bool {
        y >= self.height
    }

    pub fn count_slope(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0; 
        let mut y = 0; 
        let mut count = 0;
        while !self.is_end(y) {
            if self.is_tree(x, y) {
                count += 1;
            } 
            x += dx;
            y += dy;
        }
        count
    }
}

fn part1(map: &Map) {
    println!("Part 1 got {} trees", map.count_slope(3, 1));
}

fn part2(map: &Map) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes.iter().map(|s| map.count_slope(s.0, s.1)).product();
    println!("Part 2 product is {}", product);
}

fn main() -> anyhow::Result<()> {
    let map = Map::parse(std::io::stdin().lock().lines())?;
    part1(&map);
    part2(&map);
    Ok(())
}
