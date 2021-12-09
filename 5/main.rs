
fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let mut ones = vec![0; 12];

    for &line in input.iter() {
        for (i,c) in line.chars().enumerate() {
            match c {
                '0' => {},
                '1' => ones[i] = ones[i] + 1,
                _ => panic!("Unknown contents"),
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let half = input.len() / 2;

    for &o in ones.iter() {
        if o < half {
            gamma = gamma * 2;
            epsilon = epsilon * 2 + 1;
        } else {
            gamma = gamma * 2 + 1;
            epsilon = epsilon * 2;
        }
    }

    println!("{:?} gamma: {:012b} epsilon: {:012b} product {}", ones, gamma, epsilon, gamma * epsilon);
}

