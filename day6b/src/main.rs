fn diff(a: usize, b:usize) -> usize {
    let dist = if a > b {
        a - b
    } else {
        b - a
    };
    let mut sum = 0;
    for c in 0..=dist {
        sum += c;
    }
    sum
}

fn main() {
    let input: Vec<usize> = include_str!("input.txt")
//    let input: Vec<usize> = "16,1,2,0,4,2,7,1,2,14"
        .trim_end()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let &max = input.iter().max().unwrap();
    let mut costs: Vec<usize> = (0..=max).collect();

    for loc in 0..=max {
        costs[loc] = 0;
        for &i in input.iter() {
            costs[loc] += diff(i,loc);
        }
    }

    println!("costs: {:?}", costs);
    println!("cheapest: {}", costs.iter().min().unwrap());
}
