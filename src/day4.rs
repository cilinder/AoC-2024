use std::io;

type Grid = Vec<Vec<char>>;

fn check_bounds(grid: &Grid, row: isize, col: isize) -> bool {
    let num_rows = isize::try_from(grid.len()).unwrap();
    let num_cols = isize::try_from(grid[0].len()).unwrap();
    0 <= row && row < num_rows && 0 <= col && col < num_cols
}

fn check_xmas(grid: &Grid, row: isize, col: isize, drow: isize, dcol: isize) -> bool {
    let mut row = row;
    let mut col = col;
    for c in ['X', 'M', 'A', 'S'] {
        let in_bounds = check_bounds(grid, row, col);
        if !in_bounds {
            return false;
        }
        let row_ = usize::try_from(row).unwrap();
        let col_ = usize::try_from(col).unwrap();
        if grid[row_][col_] != c {
            return false;
        }
        row += drow;
        col += dcol;
    }
    true
}

fn check_crossmas(grid: &Grid, row: usize, col: usize) -> bool {
    if grid[row][col] != 'A' {
        return false;
    }
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    if !(0 < row  && row < num_rows - 1 && 0 < col && col < num_cols - 1) {
        return false;
    }
    let topleft = grid[row-1][col-1];
    let topright = grid[row-1][col+1];
    let bottomleft = grid[row+1][col-1];
    let bottomright = grid[row+1][col+1];
    if topleft == 'M' && bottomright == 'S' || topleft == 'S' && bottomright == 'M' {
        if bottomleft == 'M' && topright == 'S' || bottomleft == 'S' && topright == 'M' {
            return true;
        }
    }
    false
}

pub fn run() {
    let lines = io::stdin().lines();
    let mut grid: Grid = Vec::new();
    for result in lines {
        
        let line = result.expect("Could not read line");
        grid.push(line.chars().collect());
    }
    
    let directions: Vec<(isize, isize)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
    let num_rows = isize::try_from(grid.len()).unwrap();
    let num_cols = isize::try_from(grid[0].len()).unwrap();
    let mut xmas_count = 0;
    let mut crossmas_count = 0;
    for row in 0..num_rows {
        for col in 0..num_cols {
            for (drow, dcol) in directions.iter() {
                if check_xmas(&grid, row, col, *drow, *dcol) {
                    xmas_count += 1;
                }
            }
            if check_crossmas(&grid, usize::try_from(row).unwrap(), usize::try_from(col).unwrap()) {
                crossmas_count += 1;
            }
        }
    }
    
    println!("Part 1: {xmas_count}");
    println!("Part 1: {crossmas_count}");

}