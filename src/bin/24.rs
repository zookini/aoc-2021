use itertools::Itertools;

fn main() {
    let checks = include_str!("../../input/24.txt")
        .lines()
        .enumerate()
        .filter_map(|(i, line)| [5, 15].contains(&(i % 18)).then(|| line[6..].parse().unwrap()))
        .tuples::<(isize, isize)>();

    let solve = |p: fn(isize, isize) -> [isize; 2]| {
        let mut stack = vec![];
        let mut result = [0isize; 14];
    
        for (i, (a, b)) in checks.clone().enumerate() {
            if a >= 0 {
                stack.push((i, b))
            } else if let Some((j, n)) = stack.pop() {
                let [ip, jp] = p(a, n);
                
                result[i] = ip;
                result[j] = jp;
            }
        }

        result.iter().fold(0, |acc, n| acc * 10 + n)
    };

    println!("Part 1: {}", solve(|a, b| if a > -b { [9, 9 - a - b] } else { [9 + a + b, 9] }));
    println!("Part 2: {}", solve(|a, b| if a > -b { [1 + a + b, 1] } else { [1, 1 - a - b] }));
}
