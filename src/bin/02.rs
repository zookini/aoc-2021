fn main() {
    let (y, z, aim) = include_str!("../../input/02.txt")
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(direction, distance)| (direction, distance.parse::<i32>().unwrap()))
        .fold((0, 0, 0), |(y, z, aim), (direction, distance)| match direction {
            "up" => (y, z, aim - distance),
            "down" => (y, z, aim + distance),
            "forward" => (y  + aim * distance, z + distance, aim),
            _ => unreachable!()
        });

    println!("Part 1: {}", z * aim);
    println!("Part 2: {}", z * y);
}
