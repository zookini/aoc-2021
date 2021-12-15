fn main() {
    let mut risks: Vec<Vec<u8>> = include_str!("../../input/15.txt")
        .lines()
        .map(|line| line.bytes().map(|height| height - b'0').collect())
        .collect();

    let mut risks2: Vec<Vec<u8>> = (0..risks.len() * 5)
        .map(|y| (0..risks[0].len() * 5)
            .map(|x| risks[y % risks.len()][x % risks[0].len()] as usize + y / risks.len() + x / risks[0].len())
            .map(|risk| (risk.wrapping_sub(1) % 9) as u8 + 1)
            .collect()
        )
        .collect();

    println!("Part 1: {}", walk(&mut risks));
    println!("Part 2: {}", walk(&mut risks2));
}

fn walk(risks: &mut [Vec<u8>]) -> usize {
    let mut queue = std::collections::BinaryHeap::new();
    let (width, height) = (risks[0].len(), risks.len());

    std::iter::successors(Some((0, (0usize, 0usize))), |&(cost, (x, y))| {
        for (x, y) in [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)] {
            if x < width && y < height && risks[y][x] != 10 {
                queue.push(std::cmp::Reverse((cost + risks[y][x] as usize, (x, y))));
                risks[y][x] = 10;
            }
        }

        queue.pop().map(|cheapest| cheapest.0)
    })
    .find_map(|(cost, point)| (point == (width - 1, height - 1)).then(|| cost))
    .unwrap()
}
