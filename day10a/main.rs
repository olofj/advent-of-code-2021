const SCORE: [usize; 4] = [3, 57, 1197, 25137];

fn open_token(c: char) -> Option<usize> {
    match c {
        '(' => Some(0),
        '[' => Some(1),
        '{' => Some(2),
        '<' => Some(3),
        _ => None,
    }
}

fn close_token(c: char) -> Option<usize> {
    match c {
        ')' => Some(0),
        ']' => Some(1),
        '}' => Some(2),
        '>' => Some(3),
        _ => None,
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .collect();

    let mut scores: Vec<usize> = Vec::new();

    'line: for l in input {
        let mut stack: Vec<usize> = Vec::new();
        for c in l.chars() {
            if let Some(t) = open_token(c) {
                stack.push(t);
            } else if let Some(t) = close_token(c) {
                if stack.pop().unwrap() != t {
                    scores.push(SCORE[t]);
                    continue 'line;
                } 
            }
        }
    }

    println!("Sum: {}", scores.iter().sum::<usize>());
}
