const OPENS: [char ; 4] = ['(', '[', '{', '<'];
const CLOSES: [char ; 4] = [')', ']', '}', '>'];

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
                    continue 'line;
                } 
            }
        }

        let mut sum: usize = 0;
        for s in stack.iter().rev() {
            sum *= 5;
            sum += s+1;
        }

        scores.push(sum);
    }

    scores.sort();

    println!("{}", scores[scores.len()/2]);
}
