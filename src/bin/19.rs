use std::collections::HashSet;
use itertools::Itertools;

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
    let mut distances = vec![];

    while !scanners.is_empty() {
        for i in (0..scanners.len()).rev() {
            if let Some(distance) = merge(&mut connected, &scanners[i]) {
                distances.push(distance);
                scanners.remove(i);
            }
        }
    }
    
    println!("Part 1: {}", connected.len());

    println!("Part 2: {}", distances
        .iter()
        .tuple_combinations()
        .map(|([x1, y1, z1], [x2, y2, z2])| (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs())
        .max()
        .unwrap());
        
}

fn merge(connected: &mut HashSet<[isize; 3]>, scanner: &[[isize; 3]]) -> Option<[isize; 3]> {
    for i in 0..24 {
        let reoriented: Vec<_> = scanner.iter().map(|&beacon| orient(beacon, i)).collect();

        for ([x1, y1, z1], [x2, y2, z2]) in itertools::iproduct!(connected.iter(), reoriented.iter()) {
            let [dx, dy, dz] = [x1 - x2, y1 - y2, z1 - z2];
            let translated = reoriented.iter().map(|[x, y, z]| [x + dx, y + dy, z + dz]);

            if translated.clone().filter(|beacon| connected.contains(beacon)).count() >= 12 {
                connected.extend(translated);
                return Some([dx, dy, dz]);
            }
        }
    }

    None
}

fn orient([x, y, z]: [isize; 3], orientation: usize) -> [isize; 3] {
    [
        [x,  y,  z], [x,  z, -y], [x, -y, -z], [x, -z,  y], [y,  x, -z], [y,  z,  x], [y, -x,  z], [y, -z, -x],
        [z,  x,  y], [z,  y, -x], [z, -x, -y], [z, -y,  x], [-x,  y, -z], [-x,  z,  y], [-x, -y,  z], [-x, -z, -y],
        [-y,  x,  z], [-y,  z, -x], [-y, -x, -z], [-y, -z,  x], [-z,  x, -y], [-z,  y,  x], [-z, -x,  y], [-z, -y, -x],
    ][orientation]
}
