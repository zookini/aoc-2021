fn main () {
    let mut grid: Vec<Vec<_>> = include_str!("../../input/25.txt").lines().map(|s| s.bytes().collect()).collect();
    println!("{}", (1..).find(|_| step(&mut grid, b'>', (1, 0)) + step(&mut grid, b'v', (0, 1)) == 0).unwrap());
}

fn step(grid: &mut Vec<Vec<u8>>, dir: u8, (dx, dy): (usize, usize)) -> usize {
    let snapshot = grid.clone();

    itertools::iproduct!(0..grid[0].len(), 0..grid.len())
        .map(|(x, y)| (x, y, (x + dx) % snapshot[0].len(), (y + dy) % snapshot.len()))
        .filter_map(|(x, y, nx, ny)| (snapshot[y][x] == dir && snapshot[ny][nx] == b'.').then(|| {
            grid[y][x] = b'.';
            grid[ny][nx] = dir;
        }))
        .count()
}
