use itertools::Itertools;

fn main() {
    let lines: Vec<(isize, isize, isize, isize)> = include_str!("../../input/05.txt")
        .lines()
        .map(|line| line.split(|c: char| !c.is_numeric()).filter_map(|n| n.parse().ok()).collect_tuple().unwrap())
        .collect();

    println!("Part 1: {}", overlaps(lines.iter().filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)));
    println!("Part 2: {}", overlaps(lines.iter()));
}

fn overlaps<'a>(lines: impl Iterator<Item = &'a (isize, isize, isize, isize)>) -> usize {
    lines.flat_map(|&(x1, y1, x2, y2)| plot((x1, y1), (x2, y2))).counts().values().filter(|&&count| count >= 2).count()
}

fn plot((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    let (dx, dy) = (x2.cmp(&x1) as isize, y2.cmp(&y1) as isize);
    std::iter::successors(Some((x1, y1)), move |&(x, y)| ((x, y) != (x2, y2)).then(|| ((x + dx), (y + dy))))
}
