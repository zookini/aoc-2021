fn main() {
    let crabs: Vec<isize> = include_str!("../../input/07.txt").split(',').map(|n| n.parse().unwrap()).collect();

    let cheapest = |cost: fn(isize) -> isize| (*crabs.iter().min().unwrap()..*crabs.iter().max().unwrap())
        .map(|x| crabs.iter().map(|&crab| cost((crab - x).abs())).sum::<isize>())
        .min()
        .unwrap();

    println!("Part 1: {}", cheapest(|distance| distance));
    println!("Part 2: {}", cheapest(|distance| distance * (distance + 1) / 2));
}
