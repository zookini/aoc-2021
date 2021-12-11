fn main() {
    let mut grid: Vec<Vec<u8>> = include_str!("../../input/11.txt")
        .lines()
        .map(|line| line.bytes().map(|octopus| octopus - b'0').collect())
        .collect();

    println!("{}", (1..=100).map(|_| step(&mut grid)).sum::<usize>());
    println!("{}", (101..).find(|_| step(&mut grid) == grid.len() * grid[0].len()).unwrap());
}

fn step(grid: &mut [Vec<u8>]) -> usize {
    for (x, y) in itertools::iproduct!(0..grid[0].len(), 0..grid.len()) {
        flash(grid, (x, y));
    }

    grid.iter_mut().flatten().filter(|o| **o > 9).map(|o| *o = 0).count()
}

fn flash(grid: &mut [Vec<u8>], (x, y): (usize, usize)) {
    grid[y][x] += 1;

    if grid[y][x] == 10 {
        for neighbour in neighbours((x, y), grid.len()) {
            flash(grid, neighbour);
        }
    }
}

fn neighbours((x, y): (usize, usize), size: usize) -> impl Iterator<Item = (usize, usize)> {
    itertools::iproduct!(x.saturating_sub(1)..(x + 2), y.saturating_sub(1)..(y + 2))
        .filter(move |&(nx, ny)| (nx, ny) != (x, y) && nx < size && ny < size)
}
