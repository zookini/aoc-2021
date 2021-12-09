fn main() {
    let parse = || include_str!("../../input/08.txt")
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(signals, outputs)| (signals.split(' '), outputs.split(' ')));

    println!("Part 1: {}", parse().flat_map(|(_, os)| os.filter(|o| [2, 3, 4, 7].contains(&o.len()))).count());
    println!("Part 2: {}", parse().map(|(signals, outputs)| unscramble(signals, outputs)).sum::<usize>());
}

fn unscramble<'a>(signals: impl Iterator<Item = &'a str>, outputs: impl Iterator<Item = &'a str>) -> usize {
    let bitset = |segments: &str| segments.bytes().fold(0u8, |set, segment| set | 1 << (segment - b'a'));
    let lengths: std::collections::HashMap<usize, u8> = signals.map(|signal| (signal.len(), bitset(signal))).collect();

    outputs
        .map(|o| bitset(o))
        .map(|o| match (o.count_ones(), (lengths[&3] & o).count_ones(), (lengths[&4] & o).count_ones()) {
            (2, _, _) => 1,
            (3, _, _) => 7,
            (4, _, _) => 4,
            (5, 3, _) => 3,
            (5, _, 3) => 5,
            (5, _, _) => 2,
            (6, _, 4) => 9,
            (6, 3, _) => 0,
            (6, _, _) => 6,
            (7, _, _) => 8,
            _ => unreachable!()
        })
        .fold(0, |output, digit| output * 10 + digit)
}
