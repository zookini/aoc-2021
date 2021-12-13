type Dots = std::collections::HashSet<(usize, usize)>;

fn main() {
    let (dots, mut folds) = include_str!("../../input/13.txt")
        .split_once("\r\n\r\n")
        .map(|(dots, folds)| (
            dots.lines()
                .map(|line| line.split_once(',').unwrap())
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())),
            folds
                .lines()
                .map(|line| line.split_once('=').unwrap())
                .map(|(axis, fold)| (axis.chars().last().unwrap(), fold.parse().unwrap()))
        ))
        .unwrap();

    let p1 = fold(dots, folds.next().unwrap());

    println!("Part 1: {}", p1.len());
    println!("Part 2:");

    let p2 = folds.fold(p1, |dots, f| fold(dots.into_iter(), f));

    for y in 0..=*p2.iter().map(|(_, y)| y).max().unwrap() {
        println!("{}", (0..=*p2.iter().map(|(x, _)| x).max().unwrap())
            .map(|x| if p2.contains(&(x, y)) { '#' } else { ' ' })
            .collect::<String>());
    }
}

fn fold(dots: impl Iterator<Item = (usize, usize)>, (axis, n): (char, usize)) -> Dots {
    dots.map(|(x, y)| (
            if axis == 'x' && x > n { n * 2 - x } else { x },
            if axis == 'y' && y > n { n * 2 - y } else { y },
        ))
        .collect()
}
