use itertools::Itertools;

fn main () {
    let (template, rules) = include_str!("../../input/14.txt").split_once("\r\n\r\n").unwrap();
    
    let rules: std::collections::HashMap<(char, char), char> = rules
        .lines()
        .map(|line| (line.chars().next_tuple().unwrap(), line.chars().last().unwrap()))
        .collect();

    std::iter::successors(Some(template.chars().tuple_windows().counts()), |pairs| Some(pairs
            .iter()
            .flat_map(|(&(a, b), &c)| [((a, rules[&(a, b)]), c), ((rules[&(a, b)], b), c)])
            .into_grouping_map()
            .sum()
        ))
        .zip(0..=40)
        .filter(|&(_, i)| i == 10 || i == 40)
        .map(|(pairs, _)| pairs.into_iter().map(|((a, _), c)| (a, c)))
        .map(|cs| std::iter::once((template.chars().last().unwrap(), 1)).chain(cs).into_grouping_map().sum())
        .for_each(|cs| println!("{}", cs.values().max().unwrap() - cs.values().min().unwrap()));
}
