fn main() {
    let report: Vec<u16> = include_str!("../../input/01.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part 1: {}", report.windows(2).filter(|w| w[0] < w[1]).count());
    println!("Part 2: {}", report.windows(4).filter(|w| w[0] < w[3]).count());
}
