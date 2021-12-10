const NAV: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

type Color = isize;
const NO_COLOR: Color = -1 as Color;
const PEAK: Color = 0 as Color;

fn fill(map: &Vec<Vec<Color>>, (startx, starty): (usize, usize)) -> Vec<(usize, usize)> {
    let mut q: Vec<(usize,usize)> = Vec::new();
    let mut ret: Vec<(usize, usize)> = Vec::new();

    if map[startx][starty] == NO_COLOR {
        ret.push((startx, starty));
        q.push((startx, starty));
    }

    while let Some((x,y)) = q.pop() {
        'nav: for (dx, dy) in NAV {
            let xx = x as isize + dx;
            let yy = y as isize + dy;

            if xx < 0 || xx >= map.len() as isize {
                continue 'nav;
            }
            let xx = xx as usize;
            if yy < 0 || yy >= map[xx].len() as isize {
                continue 'nav;
            }
            let yy = yy as usize;
            if map[xx][yy] != PEAK && ! ret.contains(&(xx,yy)) {
                ret.push((xx,yy));
                q.push((xx,yy));
            }
        }
    }
    ret
}

fn main() {
    let input: Vec<Vec<isize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| (c as isize) - ('0' as isize)).collect())
        .collect();

    let mut map: Vec<Vec<Color>> = input
        .iter()
        .map(|l| l
             .iter()
             .map(|c| match c { 9 => PEAK, _ => NO_COLOR })
             .collect())
        .collect();

    let mut color = 1 as Color;

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let c = map[x][y];
            if c == NO_COLOR {
                for (fx, fy) in fill(&map, (x, y)) {
                    map[fx][fy] = color;
                }
                color += 1;
            }
        }
    }

    let mut hist: Vec<usize> = vec![0; color as usize];

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let color = map[x][y];
            if color == NO_COLOR {
                panic!("Never filled ({},{})", x, y);
            }
            if color != PEAK {
                hist[color as usize] += 1;
            }
        }
    }

    hist.sort_by(|a, b| b.cmp(a));

    println!("product {}", hist.iter().take(3).product::<usize>());
}
