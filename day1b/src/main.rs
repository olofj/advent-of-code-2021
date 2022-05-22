fn main() {
    let input: Vec<usize> = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    let avg: Vec<usize> = input.windows(3).map(|w| w.iter().sum::<usize>()).collect();

    let sum: usize = avg.windows(2).map(|p| (p[1] > p[0]) as usize).sum();

    println!("{}", sum);
}


