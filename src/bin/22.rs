fn main() {
    let parse = |s: &str| -> ([isize; 6], bool) {(
        s[5..].split(|c| "xyz=.,".contains(c)).filter_map(|n| n.parse().ok()).collect::<Vec<_>>().try_into().unwrap(),
        &s[..2] == "on"
    )};

    let steps = include_str!("../../input/22.txt").lines().map(parse);

    println!("Part 1: {}", solve(steps.clone().filter(|(cube, _)| cube.iter().all(|&n| -50 <= n && n <= 50))));
    println!("Part 2: {}", solve(steps));
}

fn solve(steps: impl Iterator<Item = ([isize; 6], bool)>) -> isize {
    let mut cubes = std::collections::HashMap::new();

    for step in steps {
        for seen in cubes.clone() {
            if let Some(ix) = intersection(&step.0, &seen.0) {
                *cubes.entry(ix).or_insert(0) -= seen.1 as isize;
            }
        }

        *cubes.entry(step.0).or_insert(0) += step.1 as isize;
    }

    cubes.iter().map(|([x1, x2, y1, y2, z1, z2], state)| (x2 - x1 + 1) * (y2 - y1 + 1) * (z2 - z1 + 1) * state).sum()
}

fn intersection(&[x1, x2, y1, y2, z1, z2]: &[isize; 6], &[u1, u2, v1, v2, w1, w2]: &[isize; 6]) -> Option<[isize; 6]> {
    let ix @ [x1, x2, y1, y2, z1, z2] = [x1.max(u1), x2.min(u2), y1.max(v1), y2.min(v2), z1.max(w1), z2.min(w2)];
    (x1 <= x2 && y1 <= y2 && z1 <= z2).then(|| ix)
}
