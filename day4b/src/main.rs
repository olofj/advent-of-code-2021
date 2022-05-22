#[derive(Debug,Copy,Clone)]
struct Board {
    full: bool,
    sum: u32,
    tiles: [[u32; 5]; 5],
    row_seen: [u32; 5],
    col_seen: [u32; 5],
}


fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .split("\n\n")
        .collect();

    let numbers = &input[0]
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let boards = &input[1..].iter().
        map(|b| b
            .lines()
            .map(|line| line.
                 split_whitespace()
                 .map(|v| v
                      .parse::<u32>()
                      .unwrap())
                 .collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>())
        .collect::<Vec<Vec<Vec<u32>>>>();

    let mut boards:Vec<Board> = boards.iter()
        .map(|b| Board {
            full: false,
            sum: { let mut sum = 0; for i in 0..5 { for j in 0..5 { sum += b[i][j] } } sum },
            tiles: {
                let mut t = [[0u32; 5]; 5];
                for i in 0..5 {
                    for j in 0..5 {
                        t[i][j] = b[i][j];
                    }
                }
                t
            },
            row_seen: [0; 5],
            col_seen: [0; 5],
        }).collect();

    let mut remaining = boards.len();

    for num in numbers {
        println!("num: {}", num);
        for b in 0..boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[b].tiles[i][j] == *num {
                        boards[b].row_seen[i] += 1;
                        boards[b].col_seen[j] += 1;
                        boards[b].sum -= num;
                        if boards[b].row_seen[i] == 5 || boards[b].col_seen[j] == 5 {
                            if !boards[b].full {
                                remaining -= 1;
                                boards[b].full = true;
                            }
                            if remaining == 0 {
                                println!("Last board!! board {} {:?}", b, boards[b]);
                                println!("Score: {}", num * boards[b].sum);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

}

