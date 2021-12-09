fn main() {
    let mut timer = [0u64; 9];

    let input: Vec<usize> = include_str!("input.txt")
        .trim_end()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    for &f in input.iter() {
        timer[f] += 1;
    }

    println!("start {:?}", timer);
    for d in 1..=256 {
        let new = timer[0];
        for t in 0..8 {
            timer[t] = timer[t+1];
        }
        timer[6] += new;
        timer[8] = new;
        println!("After day {}: {:?}", d, timer);
    }

    let mut sum = 0;
    for t in 0..=8 {
        sum += timer[t];
    }

    println!("Sum is {}", sum);
}
