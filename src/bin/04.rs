fn main() {
    let mut sections = include_str!("../../input/04.txt").split("\r\n\r\n");

    let draws = sections.next().unwrap().split(',').map(|n| n.parse().unwrap());

    let mut boards: Vec<Vec<Vec<Option<usize>>>> = sections
        .map(|board| board
            .lines()
            .map(|line| line.split_whitespace().map(|n| n.parse().ok()).collect())
            .collect()
        )
        .collect();

    let mut scores = draws.filter_map(|draw| (0..boards.len())
        .rev()
        .filter_map(|i| {
            boards[i].iter_mut().flatten().filter(|n| **n == Some(draw)).for_each(|c| *c = None);
            winner(&boards[i]).then(|| draw * boards.remove(i).iter().flatten().flatten().sum::<usize>())
        })
        .last()
    );

    println!("Part 1: {}", scores.next().unwrap());
    println!("Part 2: {}", scores.last().unwrap()); 
}

fn winner(board: &[Vec<Option<usize>>]) -> bool {
    (0..board.len()).any(|y| (0..board[0].len()).all(|x| board[y][x].is_none())) ||
    (0..board[0].len()).any(|x| (0..board.len()).all(|y| board[y][x].is_none()))
}
