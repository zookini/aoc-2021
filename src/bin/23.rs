fn main() {
    let mut burrow: Vec<[u8; 13]> = include_str!("../../input/23.txt")
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>().try_into().unwrap())
        .collect();

    println!("Part 1: {}", solve(&TryInto::<[[u8; 13]; 5]>::try_into(burrow.clone()).unwrap()));
    burrow.splice(3..3, [*b"  #D#C#B#A#  ", *b"  #D#B#A#C#  "]);
    println!("Part 2: {}", solve(&TryInto::<[[u8; 13]; 7]>::try_into(burrow).unwrap()));
}

fn solve<const N: usize>(burrow: &[[u8; 13]; N]) -> usize {
    let mut queue = std::collections::BinaryHeap::new();
    let mut distances = rustc_hash::FxHashMap::default();

    std::iter::successors(Some((0, *burrow)), |(cost, burrow)| {
        if distances.get(burrow).filter(|&&c| c < *cost).is_none() {
            for (cost, burrow) in walk(&burrow).map(|(c, burrow)| (cost + c, burrow)) {
                if distances.get(&burrow).filter(|&&c| c <= cost).is_none() {
                    distances.insert(burrow, cost);
                    queue.push((std::cmp::Reverse(cost), burrow));
                }
            }
        }

        queue.pop().map(|(std::cmp::Reverse(cost), burrow)| (cost, burrow))
    })
    .find_map(|(cost, burrow)| burrow[2..burrow.len() - 1].iter().all(|row| &row[3..10] == b"A#B#C#D").then(|| cost))
    .unwrap()
}

fn walk<const N: usize>(burrow: &[[u8; 13]; N]) -> impl Iterator<Item = (usize, [[u8; 13]; N])> + '_ {
    const ROOMS: [usize; 4] = [3, 5, 7, 9];

    itertools::enumerate(&burrow[1])
        .filter_map(|(x, b)| b"ABCD".contains(b).then(|| (x, ROOMS[(b - b'A') as usize], 10usize.pow((b - b'A') as u32))))
        .filter(|&(x, room, _)| (x.min(room)..=x.max(room)).all(|x2| x == x2 || burrow[1][x2] == b'.'))
        .filter_map(|(x, room, energy)| (2..)
            .find(|&y| burrow[y][room] != b'.')
            .filter(|&y| [burrow[1][x], b'#'].contains(&burrow[y][room]))
            .map(|y| ((x.max(room) - x.min(room) + y - 2) * energy, step(burrow, x, 1, room, y - 1)))
        )
        .chain(itertools::iproduct!(ROOMS, 2..burrow.len() - 1)
            .filter(|&(x, y)| b"ABCD".contains(&burrow[y][x]) && (2..y).all(|y| burrow[y][x] == b'.'))
            .flat_map(move |(x, y)| (x..)
                .take_while(|&x| burrow[1][x] == b'.')
                .chain((1..=x).rev().take_while(|&x| burrow[1][x] == b'.'))
                .filter(|x| !ROOMS.contains(x))
                .map(move |x2| ((x.max(x2) - x.min(x2) + y - 1) * 10usize.pow((&burrow[y][x] - b'A') as u32), step(burrow, x, y, x2, 1)))
            )
        )
}

fn step<const N: usize>(burrow: &[[u8; 13]; N], x0: usize, y0: usize, x1: usize, y1: usize) -> [[u8; 13]; N] {
    let mut b = burrow.clone();
    b[y1][x1] = burrow[y0][x0];
    b[y0][x0] = b'.';
    b
}
