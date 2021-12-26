use itertools::Itertools;

type HashSet<T> = rustc_hash::FxHashSet<T>;

fn main() {
    let mut scanners: Vec<Vec<[isize; 3]>> = include_str!("../../input/19.txt")
        .split("\r\n\r\n")
        .map(|scanner| scanner
            .lines()
            .skip(1)
            .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>().try_into().unwrap())
            .collect()
        )
        .collect();

    let mut connected = HashSet::from_iter(scanners.remove(0));
    let mut positions = vec![];

    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some((scanner, position)) = connection(&mut connected, &scanners[i]) {
                connected.extend(scanner);
                positions.push(position);
                scanners.remove(i);
            }
        }
    }

    println!("Part 1: {}", connected.len());

    println!("Part 2: {}", positions
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs())
        .max()
        .unwrap());
        
}

fn connection(connected: &HashSet<[isize; 3]>, scanner: &[[isize; 3]]) -> Option<(Vec<[isize; 3]>, [isize; 3])> {
    (0..24)
        .map(|i| scanner.iter().map(|&beacon| orient(beacon, i)).collect::<Vec<_>>())
        .find_map(|reoriented| itertools::iproduct!(connected.iter(), reoriented.iter())
            .map(|([x1, y1, z1], [x2, y2, z2])| ([x1 - x2, y1 - y2, z1 - z2]))
            .map(|[dx, dy, dz]| (reoriented.iter().map(move |[x, y, z]| [x + dx, y + dy, z + dz]), [dx, dy, dz]))
            .find(|(translated, _)| (translated.clone().filter(|beacon| connected.contains(beacon)).count() >= 12))
            .map(|(translated, position)| (translated.collect(), position))
        )
}

fn orient([x, y, z]: [isize; 3], orientation: usize) -> [isize; 3] {
    [
        [x,  y,  z], [x,  z, -y], [x, -y, -z], [x, -z,  y], [y,  x, -z], [y,  z,  x], [y, -x,  z], [y, -z, -x],
        [z,  x,  y], [z,  y, -x], [z, -x, -y], [z, -y,  x], [-x,  y, -z], [-x,  z,  y], [-x, -y,  z], [-x, -z, -y],
        [-y,  x,  z], [-y,  z, -x], [-y, -x, -z], [-y, -z,  x], [-z,  x, -y], [-z,  y,  x], [-z, -x,  y], [-z, -y, -x],
    ][orientation]
}
