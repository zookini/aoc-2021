fn main() {
    println!("Part 1: {}", linear(1, 10, 0, 0, 1));
    println!("Part 2: {}", quantum(1, 10, 0, 0).iter().max().unwrap());
}

fn linear(p1: usize, p2: usize, s1: usize, s2: usize, roll: usize) -> usize {
    if s2 >= 1000 {
        s1 * (roll - 1)
    } else {
        let p1 = (p1 + roll * 3 + 2) % 10 + 1;
        linear(p2, p1, s2, s1 + p1, roll + 3)
    }
}

fn quantum(p1: usize, p2: usize, s1: usize, s2: usize) -> [usize; 2] {
    if s2 >= 21 {
        [0, 1]
    } else {
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].iter().fold([0, 0], |total, (rolls, frequency)| {
            let p1 = (p1 + rolls - 1) % 10 + 1;
            let score = quantum(p2, p1, s2, s1 + p1);
            [total[0] + score[1] * frequency, total[1] + score[0] * frequency]
        })
    }
}
