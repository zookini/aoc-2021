fn main() {
    let mut fishes = [0; 9];

    for i in include_str!("../../input/06.txt").split(',') {
        fishes[i.parse::<usize>().unwrap()] += 1;
    }

    println!("Part 1: {}", solve(fishes, 80));
    println!("Part 2: {}", solve(fishes, 256));
}

fn solve(mut fishes: [usize; 9], n: usize) -> usize {
    for i in 1..=n {
        fishes[(6 + i) % fishes.len()] += fishes[(8 + i) % fishes.len()];
    }

    fishes.iter().sum()
}
