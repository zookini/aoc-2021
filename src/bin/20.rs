fn main() {
    let (iea, image) = include_str!("../../input/20.txt").split_once("\r\n\r\n").unwrap();
    let image: Vec<Vec<bool>> = image.lines().map(|line| line.chars().map(|c| c == '#').collect()).collect();

    let pixel = |image: &[Vec<bool>], (x, y), default| iea.as_bytes()[itertools::iproduct!(y - 1..y + 2, x - 1..x + 2)
        .fold(0, |n, (y, x)| n * 2 + if x >= 0 && y >= 0 && x < image[0].len() as isize && y < image.len() as isize {
            image[y as usize][x as usize] as usize
        } else {
            default as usize
        })] == b'#';

    let enhance = |image: &[Vec<bool>], default| itertools::iproduct!(0..image.len() + 2, 0..image[0].len() + 2)
        .fold(vec![vec![false; image[0].len() + 2]; image.len() + 2], |mut enhanced, (y, x)| {
            enhanced[y][x] = pixel(image, (x as isize - 1, y as isize - 1), default);
            enhanced    
        });

    std::iter::successors(Some((image, false)), |(image, default)| Some((enhance(&image, *default), !default)))
        .zip(0..=50)
        .filter_map(|((image, _), i)| (i == 2 || i == 50).then(|| image))
        .for_each(|image| println!("{}", image.iter().flatten().filter(|&&b| b).count()));
}
