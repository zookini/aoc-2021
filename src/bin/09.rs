use itertools::Itertools;

fn main() {
    let mut heights: Vec<Vec<u8>> = include_str!("../../input/09.txt")
        .lines()
        .map(|line| line.bytes().map(|height| height - b'0').collect())
        .collect();

    let p1: usize = itertools::iproduct!(0..heights[0].len(), 0..heights.len())
        .filter(|&(x, y)| neighbours((x, y), heights.len()).all(|(x2, y2)| heights[y][x] < heights[y2][x2]))
        .map(|(x, y)| heights[y][x] as usize + 1)
        .sum();

    println!("Part 1: {}", p1);

    let p2: usize = itertools::iproduct!(0..heights[0].len(), 0..heights.len())
        .map(|point| basin(&mut heights, point))
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product();

    println!("Part 2: {}", p2);
}

fn neighbours((x, y): (usize, usize), size: usize) -> impl Iterator<Item = (usize, usize)> {
    [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)]
        .into_iter()
        .filter(move |(x, y)| x < &size && y < &size)
}

fn basin(heights: &mut [Vec<u8>], (x, y): (usize, usize)) -> usize {
    if heights[y][x] == 9 {
        0
    } else {
        heights[y][x] = 9;
        1 + neighbours((x, y), heights.len()).map(|point| basin(heights, point)).sum::<usize>()
    }
}
