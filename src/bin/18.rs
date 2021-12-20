use itertools::Itertools;
use std::iter::once;

use Element::*;

fn main() {
    let input: Vec<Vec<Element>> = include_str!("../../input/18.txt")
        .lines()
        .map(|line| line.chars().map(parse).collect())
        .collect();

    println!("Part 1: {}", magnitude(&input.iter().cloned().reduce(|a, b| add(a, b)).unwrap()));
    println!("Part 2: {}", input.iter().permutations(2).map(|p| magnitude(&add(p[0].clone(), p[1].clone()))).max().unwrap());
}

#[derive(Copy, Clone, Debug)]
enum Element { Open, Close, Comma, Num(usize) }

fn parse(c: char) -> Element {
    match c {
        '[' => Open,
        ']' => Close,
        ',' => Comma,
        n => Num((n as u8 - b'0') as usize)
    }
}

fn magnitude(pairs: &[Element]) -> usize {
    pairs.iter().fold((0, 1), |(magnitude, multiplier), element| match element {
        Open => (magnitude, multiplier * 3),
        Close => (magnitude, multiplier / 2),
        Comma => (magnitude, multiplier / 3 * 2),
        Num(n) => (magnitude + multiplier * n, multiplier),
    }).0
}

fn add(a: Vec<Element>, b: Vec<Element>) -> Vec<Element> {
    reduce(once(Open).chain(reduce(a)).chain(once(Comma)).chain(reduce(b)).chain(once(Close)).collect())
}

fn reduce(mut pairs: Vec<Element>) -> Vec<Element> {
    while explode(&mut pairs) || split(&mut pairs) { }
    pairs
}

fn explode(pairs: &mut Vec<Element>) -> bool {
    let mut depth = 0;

    pairs
        .iter()
        .position(|e| {
            depth += if let Open = e { 1 } else if let Close = e { -1 } else { 0 };
            depth == 5
        })
        .map(|i| if let (Num(a), Num(b)) = (pairs[i + 1], pairs[i + 3]) {
            pairs[..i].iter_mut().rev().find_map(|n| if let Num(n) = n { Some(n) } else { None }).map(|n| *n += a);
            pairs[i + 4..].iter_mut().find_map(|n| if let Num(n) = n { Some(n) } else { None }).map(|n| *n += b);
            pairs.splice(i..i + 5, [Num(0)]);
        })
        .is_some()
}

fn split(pairs: &mut Vec<Element>) -> bool {
    pairs
        .iter()
        .position(|e| match e { Num(n) => *n > 9, _ => false })
        .map(|i| if let Num(n) = pairs[i] { pairs.splice(i..i + 1, [Open, Num(n / 2), Comma, Num((n + 1) / 2), Close]); })
        .is_some()
}
