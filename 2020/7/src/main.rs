use hashbrown::HashMap;
use hashbrown::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref RE_START: Regex = Regex::new(r"^(\w+ \w+) bags contain ").unwrap();
    static ref RE_CONTAINED: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

#[derive(Copy, Clone)]
enum EdgeDirection {
    Contained,
    Contains,
}

struct Contains<'a> {
    container_color: &'a str,
    contained_color: &'a str,
    contained_count: u32,
}

struct Graph {
    contains: HashMap<String, HashMap<String, u32>>,
    contained_by: HashMap<String, HashMap<String, u32>>,
}

fn add_edge(edges: &mut HashMap<String, HashMap<String, u32>>, from: &str, to: &str, value: u32) {
    let entry = edges
        .raw_entry_mut()
        .from_key(from)
        .or_insert_with(|| (from.to_owned(), HashMap::new()));
    entry.1.insert(to.to_owned(), value);
}

fn for_each_edge<'a, F>(edges: &'a HashMap<String, HashMap<String, u32>>, from: &str, mut f: F)
where
    F: FnMut(&'a String, u32),
{
    edges.get(from).map(move |edges| {
        edges.iter().for_each(|kv| {
            f(kv.0, *kv.1);
        });
    });
}

impl Graph {
    fn new() -> Graph {
        Graph {
            contains: HashMap::new(),
            contained_by: HashMap::new(),
        }
    }

    fn add(&mut self, c: Contains) {
        add_edge(
            &mut self.contained_by,
            c.contained_color,
            c.container_color,
            c.contained_count,
        );
        add_edge(
            &mut self.contains,
            c.container_color,
            c.contained_color,
            c.contained_count,
        );
    }

    fn for_each_edge<'a, F>(&'a self, from: &str, direction: EdgeDirection, f: F)
    where
        F: FnMut(&'a String, u32),
    {
        let edges: &HashMap<_, _> = match direction {
            EdgeDirection::Contained => &self.contained_by,
            EdgeDirection::Contains => &self.contains,
        };
        for_each_edge(edges, from, f);
    }

    fn dfs_post_order<'a, F>(&'a self, from: &'a str, direction: EdgeDirection, f: &mut F)
    where
        F: FnMut(&'a str),
    {
        let mut visited: HashSet<&str> = HashSet::new();
        self.do_dfs_post_order(from, direction, &mut |n| {
            if visited.insert(n) {
                f(n);
            }
        });
    }

    // Recursive because the iterative version is a pain in the ass to write.
    fn do_dfs_post_order<'a, F>(&'a self, from: &'a str, direction: EdgeDirection, f: &mut F)
    where
        F: FnMut(&'a str),
    {
        self.for_each_edge(from, direction, |neighbour, _| {
            self.do_dfs_post_order(neighbour, direction, f);
        });
        f(from);
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = Contains> {
    let start_cap = RE_START.captures(line).unwrap();
    let container_color = start_cap.get(1).unwrap().as_str();
    let contained_slice = &line[start_cap.get(0).unwrap().end()..];
    let contained_iter = RE_CONTAINED.captures_iter(contained_slice);
    contained_iter.map(move |cap| {
        let contained_count: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let contained_color: &str = cap.get(2).unwrap().as_str();
        Contains {
            container_color,
            contained_color,
            contained_count,
        }
    })
}

fn main() {
    let mut graph = Graph::new();
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .for_each(|l| {
            parse_line(&l).for_each(|contains| graph.add(contains));
        });

    // part 1
    let mut count = 0;
    graph.dfs_post_order("shiny gold", EdgeDirection::Contained, &mut |_| {
        count += 1;
    });
    // -1 because dfs post order includes the starting node
    println!("Shiny gold bags can be contained in {} bags", count - 1);

    // part 2
    let mut totals: HashMap<&str, u32> = HashMap::new();
    graph.dfs_post_order("shiny gold", EdgeDirection::Contains, &mut |n| {
        let mut sum = 0;
        graph.for_each_edge(n, EdgeDirection::Contains, |sub, count| {
            let sub_sum = totals.get(sub.as_str()).map(Clone::clone).unwrap_or(0);
            sum += (sub_sum + 1) * count;
        });
        totals.insert(n, sum);
    });
    println!("Shiny gold bags contain {} total bags", totals.get("shiny gold").unwrap());
}
