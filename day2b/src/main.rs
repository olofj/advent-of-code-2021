fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap());

    let mut fwd = 0;
    let mut aim = 0;
    let mut depth = 0;

    for (a,b) in input {
        let val: i32 = b.parse().unwrap();
        match a {
            "forward" => { fwd += val; depth += val * aim },
            "down" => aim += val,
            "up" => aim -= val,
            _ => panic!("Invalid input"),
        }
        println!("{} {} aim {} fwd {} depth {}", a, b, aim, fwd, depth);
    }
    println!("fwd {} depth {} - product {}", fwd, depth, fwd*depth);
}

