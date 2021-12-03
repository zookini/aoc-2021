fn main() {
    let report: Vec<_> = include_str!("../../input/03.txt").lines().collect();

    let (gamma, epsilon) = (0..report[0].len())
        .map(|bit| commonest(&report, bit) as usize)
        .fold((0, 0), |(g, e), c| (g * 2 + c, (e * 2 + c) ^ 1));

    println!("Part 1: {}", gamma * epsilon);
    println!("Part 2: {}", rate(report.clone(), false) * rate(report, true));
}

fn commonest(nums: &[&str], bit: usize) -> bool {
    let count = |b| nums.iter().filter(|num| num.as_bytes()[bit] == b).count();
    count(b'1') >= count(b'0')
}

fn rate(mut nums: Vec<&str>, invert: bool) -> usize {
    (0..).find(|&bit| {
        let c = (commonest(&nums, bit) as u8 ^ invert as u8) + b'0';
        nums.retain(|num| num.as_bytes()[bit] == c);
        nums.len() == 1
    });

    usize::from_str_radix(nums[0], 2).unwrap()
}
