
use std::cmp::max;

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a-b
    } else {
        b-a
    }
}

fn range(a: usize, b: usize, len: usize) -> Vec<usize> {
    let mut r: Vec<usize> = if a <= b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    };
    r.resize(len, a);
    r
}

fn main() {
    let mut board = [[0u32; 1024]; 1024];

    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    for l in input {
        let points = l.split_once(" -> ").unwrap();
        let (x1, y1) = points.0.split_once(",").unwrap();
        let (x2, y2) = points.1.split_once(",").unwrap();
        let (x1, y1) = (x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap());
        let (x2, y2) = (x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap());

        let maxlen = max(abs_diff(x1, x2), abs_diff(y1, y2)) + 1;

        let xrange = range(x1, x2, maxlen);
        let yrange = range(y1, y2, maxlen);

        for (&x,y) in xrange.iter().zip(yrange) {
            board[x][y] += 1;
        }
    }

    let mut sum = 0;
    for i in 0..1024 {
        for j in 0..1024 {
            if board[i][j] > 1 {
                sum += 1;
            }
        }
    }
    println!("sum {}", sum);
}
