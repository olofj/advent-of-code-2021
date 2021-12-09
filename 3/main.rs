fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap());

    let mut fwd = 0;
    let mut depth = 0;

    for (a,b) in input {
        let val: i32 = b.parse().unwrap();
        match a {
            "forward" => fwd += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => panic!("Invalid input"),
        }
        println!("{} {}", a, b);
    }
    println!("fwd {} depth {} - product {}", fwd, depth, fwd*depth);
}

