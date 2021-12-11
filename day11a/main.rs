const NAV: [(isize, isize); 8] = [(-1, -1), (-1, 0), (0, -1), (-1, 1), (1, -1), (1, 1), (1, 0), (0, 1)];

const XSIZE: isize = 10;
const YSIZE: isize = 10;
const STEPS: usize = 100;

fn navs(x: isize, y: isize) -> Vec<(usize,usize)> {
    let n = NAV.iter()
        .map(|(a,b)| (a+x, b+y))
        .filter(|(a,b)| a >= &0 && a < &XSIZE && b >= &0 && b < &YSIZE)
        .map(|(a,b)| (a as usize, b as usize))
        .collect();
    n
}

fn main() {
    let mut input: Vec<Vec<isize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| (c as isize) - ('0' as isize)).collect())
        .collect();
    
    let mut f = 0;

    for s in 0..STEPS {
        let mut flashes: Vec<(usize, usize)> = Vec::new();
        for x in 0..input.len() {
            for y in 0..input[x].len() {
                input[x][y] += 1;
                if input[x][y] == 10 {
                    input[x][y] = 11;
                    flashes.push((x,y));
                }
            }
        }

        while let Some((x,y)) = flashes.pop() {
            for (x, y) in navs(x as isize, y as isize) {
                input[x][y] += 1;
                if input[x][y] == 10 {
                    flashes.push((x,y));
                }
            }
        }
        for x in 0..input.len() {
            for y in 0..input[x].len() {
                if input[x][y] > 9 {
                    input[x][y] = 0;
                    f += 1;
                }
            }
        }
    }

    println!("{}", f);
}
