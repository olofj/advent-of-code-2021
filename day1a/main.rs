fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let sum: usize = input.windows(2).map(|p| if p[1] > p[0] { 1 } else { 0 }).sum();

    println!("{}", sum);
}

