use std::collections::HashMap;

fn sort(s: &str) -> String {
    let slice: &str = &s[..];

    let mut chars: Vec<char> = slice.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    chars.into_iter().collect::<String>()
}

fn filter(s1: &String, s2: &String) -> String {
    let mut ret: Vec<char> = Vec::new();
    for c in s1.chars() {
        if ! s2.contains(c) {
            ret.push(c);
        }
    }
    ret.into_iter().collect::<String>()
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let mut result: Vec<usize> = Vec::new();

    for l in input {
        let words: Vec<String> = l.split_whitespace().map(|w| sort(w)).filter(|w| w != "|").collect();
        let mut numbers: Vec<String> = Vec::new();

        for _ in 0..10 {
            numbers.push(String::from(""));
        }
        for w in &words {
            match w.len() {
                2 => numbers[1] = w.clone(), // 1
                3 => numbers[7] = w.clone(), // 7
                4 => numbers[4] = w.clone(), // 4
                7 => numbers[8] = w.clone(), // 8
                _ => {
                }
            };
        }
        for w in &words {
            let m1 = filter(&w, &numbers[1]).len();
            let m4 = filter(&w, &numbers[4]).len();
            let m7 = filter(&w, &numbers[7]).len();
            match (w.len(), m1, m4, m7) {
                (5,4,2,3) => {numbers[5] = w.clone()},
                (5,4,3,3) => {numbers[2] = w.clone()},
                (5,3,2,2) => {numbers[3] = w.clone()},
                (6,4,2,3) => {numbers[9] = w.clone()},
                (6,5,3,4) => {numbers[6] = w.clone()},
                (6,4,3,3) => {numbers[0] = w.clone()},
                _ => {},
            }
        }
        let mut nmap = HashMap::new();
        for i in 0..10 {
            println!("number[{}]: {:?}", i, numbers[i]);
            nmap.insert(numbers[i].clone(), i);
        }
        let outp: Vec<String> = l.split_once(" | ").unwrap().1.split(" ").map(|w| sort(w)).collect();
        let mut val = 0;
        for num in outp {
            println!("{} : {:?}", num, nmap[&num]);
            val *= 10;
            val += nmap[&num];
        }
        println!("val: {}", val);
        result.push(val);
    }
    let mut sum = 0;
    for r in result {
        sum += r;
    }
    println!("Sum: {}", sum);
}
