fn main() {
    let input: Vec<Vec<&str>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split_once(" | ").unwrap().1.split(" ").collect())
        .collect();

    let mut sum = 0;
    for l in input {
        for w in l {
            sum += match w.len() {
                2 => 1,
                3 => 1,
                4 => 1,
                7 => 1,
                _ => 0,
            };
        }
    }

    println!("sum {}", sum);
}
