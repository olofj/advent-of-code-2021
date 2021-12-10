fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let sum: usize = input.windows(2).map(|p| (p[1] > p[0]) as usize).sum();

    println!("{}", sum);
}

