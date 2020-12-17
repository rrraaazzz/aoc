use std::io::BufRead;

fn part1(sorted_jolts: &Vec<u32>) {
    let deltas = sorted_jolts
        .windows(2)
        .map(|w| w[1] - w[0]);
    let count1 = deltas.clone().filter(|d| *d == 1).count();
    let count3 = deltas.filter(|d| *d == 3).count();
    println!(
        "1-jolt differences multiplied by 3-jolt differences: {}",
        count1 * count3
    );
}

fn part2(sorted_jolts: &Vec<u32>) {
    // This was not obvious at all. We walk the list of adapters keeping
    // track of how many possible arrangements there are so far that end
    // with a gap of 0, a gap of 1, or a gap of 2. This is stored in
    // end0, end1, and end2 respectively.
    //
    // A gap of 0 means that from the last N adapters, the last one is used.
    //
    // A gap of 1 means that adapter N is not used, and adapter N-1 has an
    // output of 1 jolt less than adapter N.
    //
    // A gap of 2 means that adapter N is not used, and the previously used
    // adapter (which can be N-1 or N-2) has an output of 2 jolts less than
    // adapter N.
    //
    // For every new adapter, the jolt delta can be 1, 2, or 3. For each of
    // the three possibilities we compute how many arrangements there are
    // for N+1 adapters that have gaps of 0, 1, or 2. In all cases this is
    // a function of the arrangement counts for N adapters.
    let mut end0: u64 = 1;
    let mut end1: u64 = 0;
    let mut end2: u64 = 0;
    sorted_jolts.windows(2).map(|w| w[1] - w[0]).for_each(|delta| {
        let (new_end0, new_end1, new_end2) = match delta {
            1 => (end0 + end1 + end2, end0, end1),
            2 => (end0 + end1, 0, end0),
            3 => (end0, 0, 0),
            _ => panic!("Can't happen")
        };
        end0 = new_end0;
        end1 = new_end1;
        end2 = new_end2;
    });
    println!("Possible arrangements: {}", end0);
}

fn main() {
    // Add an initial joltage of 0.
    let mut jolts: Vec<u32> = vec![0];
    jolts.extend(
        std::io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .map(|l| l.parse::<u32>().unwrap()),
    );
    jolts.sort();

    // Add the final joltage of MAX + 3
    jolts.push(jolts.last().unwrap_or(&0) + 3);

    part1(&jolts);
    part2(&jolts);
}
