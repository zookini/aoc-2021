fn main() {
    let [tx1, tx2, ty1, ty2]: [i32; 4] = [143, 177, -106, -71];

    let shoot = |(dx, dy)| std::iter::successors(Some(((0, 0), (dx, dy))), |&((x, y), (dx, dy))|
            (x <= tx2 && y >= ty1).then(|| ((x + dx, y + dy), (0.max(dx - 1), dy - 1)))
        )
        .find_map(|((x, y), _)| (tx1 <= x && x <= tx2 && ty1 <= y && y <= ty2).then(|| (x, y)));

    let hits: Vec<_> = itertools::iproduct!(0..=tx2, ty1..=ty1.abs()).filter_map(shoot).collect();

    println!("Part 1: {}", hits.iter().map(|(_, y)| y * (y + 1) / 2).max().unwrap());
    println!("Part 2: {}", hits.len());
}
