const NAV: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

type Color = u32;
const NO_COLOR: Color = 0 as Color;
const XSIZE: usize = 100;
const YSIZE: usize = 100;
const MAX_COLOR: usize = (XSIZE/2)*(YSIZE/2);

fn fill(map: &Vec<Vec<isize>>,
        fill: &mut [[Color; YSIZE]; XSIZE],
        (startx, starty): (usize, usize),
        color: Color) {
    let mut q:Vec<(usize,usize)> = Vec::new();
    if fill[startx][starty] == NO_COLOR &&
        map[startx][starty] != 9 {
        fill[startx][starty] = color;
        q.push((startx, starty));
    }

    while let Some((x,y)) = q.pop() {
        for (dx, dy) in NAV {
            let xx = x as isize + dx;
            let yy = y as isize + dy;

            if xx < 0 || xx >= XSIZE as isize {
                continue;
            }
            if yy < 0 || yy >= YSIZE as isize {
                continue;
            }
            let xx = xx as usize;
            let yy = yy as usize;
            if map[xx][yy] != 9 && fill[xx][yy] == NO_COLOR {
                fill[xx][yy] = color;
                q.push((xx,yy));
            }
        }
    }
}

fn main() {
    let input: Vec<Vec<isize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| (c as isize) - ('0' as isize)).collect())
        .collect();

    let plot = &mut [[NO_COLOR; YSIZE]; XSIZE];
    let mut color = 1 as Color;
    for x in 0..XSIZE {
        for y in 0..YSIZE {
            if plot[x][y] == NO_COLOR && input[x][y] != 9 {
                fill(&input, plot, (x, y), color);
                color += 1;
            }
        }
    }

    let mut hist = [0; MAX_COLOR];

    for x in 0..XSIZE {
        for y in 0..YSIZE {
            let color = plot[x][y];
            if color != NO_COLOR {
                hist[color as usize] += 1;
            }
        }
    }

    let mut hvec: Vec<(usize, &usize)> = hist.iter().enumerate().collect();
    hvec.sort_by(|(_, av), (_, bv)| bv.cmp(av));

    println!("product {}", hvec[0].1 * hvec[1].1 * hvec[2].1);
}
