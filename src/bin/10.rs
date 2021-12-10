fn main() {
    let (mut p1, mut p2) = (0, Vec::new());

    for (line, mut stack) in include_str!("../../input/10.txt").lines().map(|line| (line, Vec::new())) {
        match line.chars().find_map(|c| parse(c, &mut stack)) {
            Some(c) => p1 += [3, 25137, 57, 1197][c as usize / 30 - 1],
            None => p2.push(stack.iter().rev().fold(0usize, |n, &c| n * 5 + [1, 4, 2, 3][c as usize / 30 - 1]))
        }
    }

    let mid = p2.len() / 2;

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2.select_nth_unstable(mid).1);
}

fn parse(c: char, stack: &mut Vec<char>) -> Option<char> {
    if "([{<".contains(c) {
        stack.push(c);
    } else if ![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')].contains(&(stack.pop().unwrap(), c)) {
        return Some(c);
    }

    None
}
