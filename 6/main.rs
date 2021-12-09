use std::iter::FromIterator;

fn common(input: &Vec<Vec<char>>, idx: usize) -> char {
    let mut ones = 0;
    let mut zeroes = 0;

    for line in input.iter() {
        match line[idx] {
            '0' => zeroes = zeroes + 1,
            '1' => ones = ones + 1,
            _ => panic!("Unknown contents"),
        }
    }

    if ones >= zeroes { '1' } else { '0' }
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut o2 = input.clone();
    let mut co2 = input.clone();

    for i in 0..12 {
        let cv = common(&o2, i);
        if o2.len() > 1 {
            o2.retain(|chars| chars[i] == cv)
        }
        let cv = common(&co2, i);
        if co2.len() > 1 {
            co2.retain(|chars| chars[i] != cv)
        }
    }

    let o2s = String::from_iter(&o2[0]);
    let o2 = usize::from_str_radix(&o2s, 2).unwrap();
    let co2s = String::from_iter(&co2[0]);
    let co2 = usize::from_str_radix(&co2s, 2).unwrap();
    println!("o2 {} {} co2 {} {} product {}", o2s, o2, co2s, co2, o2 * co2);

}

