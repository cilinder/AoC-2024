use std::io;
use std::fmt::Display;

type Grid = Vec<Vec<char>>;

fn print_grid(grid: &Grid) {
    for row in grid {
        for elt in row {
            print!("{elt}");
        }
        println!();
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::North),
            '>' => Some(Self::East),
            'v' => Some(Self::South),
            '<' => Some(Self::West),
            _ => None
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North
        }
    }
}

#[derive(Debug)]
struct State {
    x: usize,
    y: usize,
    dir: Direction
}

fn move(grid: &Grid, state: State) -> State {
    ...
}

pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let mut grid: Grid = Grid::new();
    // let mut start: (usize, usize);
    let mut state: State = State {x: 0, y: 0, dir: Direction::North};
    // let mut direction
    for (i, line) in lines.enumerate() {
        grid.push(line.chars().collect());
        match line.find(|c: char| c == '^' || c == '<' || c == 'v' || c == '>') {
            Some(j) => {
                state = State {
                    x: j,
                    y: i,
                    dir: Direction::from_char(grid[i][j]).unwrap()
                };
            },
            None => {}
        };
        
    }
    println!("{state:?}");
    print_grid(&grid);

}