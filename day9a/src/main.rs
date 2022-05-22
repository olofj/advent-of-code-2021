const NAV: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn main() {
    let input: Vec<Vec<isize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| (c as isize) - ('0' as isize)).collect())
        .collect();

    let mut lows: Vec<isize> = Vec::new();

    for i in 0..input.len() {
        'grid: for j in 0..input[i].len() {
            let v = input[i][j];
            let i = i as isize;
            let j = j as isize;
            let mut checks = Vec::new();
            for (x,y) in NAV {
                let xx = i+x;
                let yy = j+y;
                if xx < 0 || yy < 0 {
                    continue;
                }
                if xx >= input.len() as isize {
                    continue;
                }
                if yy >= input[i as usize].len() as isize {
                    continue;
                }
                let cmp = input[(i+x) as usize][(j+y) as usize];
                checks.push(cmp);
                if cmp <= v {
                    continue 'grid;
                }
            }
            println!("low point: {} {}: {} | checks {:?}", i, j, v, checks);
            lows.push(v);
        }
    }

    let mut sum = 0;
    for l in lows {
        sum += l+1;
    }
    println!("sum {}", sum);
}
