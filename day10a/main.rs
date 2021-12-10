const OPENS: [char ; 4] = ['(', '[', '{', '<'];
const CLOSES: [char ; 4] = [')', ']', '}', '>'];
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
            if OPENS.contains(&c) {
                stack.push(open_token(c).unwrap());
            } else if CLOSES.contains(&c) {
                let tok = close_token(c).unwrap();
                if stack.pop().unwrap() != tok {
                    scores.push(SCORE[tok]);
                    continue 'line;
                } 
            }
        }
    }

    println!("Sum: {}", scores.iter().sum::<usize>());
}
