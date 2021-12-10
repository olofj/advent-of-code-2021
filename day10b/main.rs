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
                    continue 'line;
                } 
            }
        }

        let mut sum: usize = 0;
        while let Some (s) = stack.pop() {
            sum *= 5;
            sum += s+1;
        }

        scores.push(sum);
    }

    scores.sort();

    println!("{}", scores[scores.len()/2]);
}
