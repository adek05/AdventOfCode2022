use crate::utils::read_lines;

pub fn next(p: (i32, i32), dp: (i32, i32)) -> (i32, i32) {
    (p.0 + dp.0, p.1 + dp.1)
}

pub fn is_out_of_grid(p: (i32, i32), dimensions: (usize, usize)) -> bool {
    p.0 < 0 || p.0 >= dimensions.0 as i32 || p.1 < 0 || p.1 >= dimensions.1 as i32
}

pub fn bfs(
    start: (i32, i32),
    dimensions: (usize, usize),
    direction: (i32, i32),
    grid: &Vec<Vec<u32>>,
    visibility: &mut Vec<Vec<bool>>,
) {
    let mut tree_loc = start;
    visibility[tree_loc.0 as usize][tree_loc.1 as usize] = true;
    let mut prev_max = grid[tree_loc.0 as usize][tree_loc.1 as usize];
    loop {
        tree_loc = next(tree_loc, direction);
        if is_out_of_grid(tree_loc, dimensions) {
            return;
        }
        let cur_tree = grid[tree_loc.0 as usize][tree_loc.1 as usize];
        if prev_max < cur_tree {
            visibility[tree_loc.0 as usize][tree_loc.1 as usize] = true;
            prev_max = cur_tree
        }
    }
}

pub fn bfs_2(start: (i32, i32), dimensions: (usize, usize), grid: &Vec<Vec<u32>>) -> i32 {
    let mut total_score = 1;
    for direction in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let mut score = 0;
        let mut tree_loc = start;
        let start_tree_height = grid[tree_loc.0 as usize][tree_loc.1 as usize];
        loop {
            tree_loc = next(tree_loc, direction);
            if is_out_of_grid(tree_loc, dimensions) {
                break;
            }
            let cur_tree = grid[tree_loc.0 as usize][tree_loc.1 as usize];
            if cur_tree < start_tree_height {
                score += 1;
            } else {
                score += 1;
                break;
            }
        }
        total_score = total_score * score;
    }
    total_score
}

pub fn run() {
    let lines = read_lines("in/day8.in").unwrap();
    let mut grid: Vec<Vec<u32>> = vec![];
    let mut visibility: Vec<Vec<bool>> = vec![];
    for line in lines {
        let l = line.unwrap();
        grid.push(l.chars().map(|c| c.to_digit(10).unwrap()).collect());
        visibility.push(l.chars().map(|c| false).collect());
    }
    let xsize = grid[0].len();
    let ysize = grid.len();

    for x in 0..xsize {
        bfs(
            (x as i32, 0),
            (xsize, ysize),
            (0, 1),
            &grid,
            &mut visibility,
        );
        bfs(
            (x as i32, (ysize as i32) - 1),
            (xsize, ysize),
            (0, -1),
            &grid,
            &mut visibility,
        );
    }
    for y in 0..ysize {
        bfs(
            (0, y as i32),
            (xsize, ysize),
            (1, 0),
            &grid,
            &mut visibility,
        );
        bfs(
            ((xsize as i32) - 1, y as i32),
            (xsize, ysize),
            (-1, 0),
            &grid,
            &mut visibility,
        );
    }
    // for x in 0..xsize {
    //     let mut prev = grid[x][xsize-1];
    //     visibility[x][0] = true;
    //     for y in 1..ysize {
    //         if grid[x][y] > prev {
    //             visibility[x][y] = true;
    //             prev = grid[x][y]
    //         } else {
    //             break;
    //         }
    //     }
    // }

    let mut cnt = 0;
    for x in 0..xsize {
        for y in 0..ysize {
            if visibility[x][y] == true {
                cnt += 1;
            }
        }
    }
    // dbg!(&visibility);
    println!("Day 8, part 1 {}", cnt);

    let mut max_score = 0;
    for x in 0..xsize {
        for y in 0..ysize {
            max_score = std::cmp::max(
                max_score,
                bfs_2((x as i32, y as i32), (xsize, ysize), &grid),
            );
        }
    }
    println!("Day 8, part 2 {}", max_score);
}
