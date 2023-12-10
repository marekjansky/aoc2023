use std::collections::{HashMap, VecDeque};



fn main() {
    // let input = include_str!("./input.txt");
     
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";

    let output = process1(input);
    println!("Result of computation: {output}");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CellType {
    None,
    Start,
    Vertical,
    Horizontal,
    SouthWest,
    SouthEast,
    NorthWest,
    NorthEast
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Position {
    North,
    South,
    West,
    East
}

impl From<char> for CellType {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '|' => Self::Horizontal,
            '-' => Self::Vertical,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'J' => Self::NorthWest,
            'L' => Self::NorthEast,
            _ => Self::None
        }
    }
}

impl CellType {
    fn connects(&self, other: &CellType, relative_position : Position) -> bool {
        match self {
            Self::Horizontal  
            Self::Vertical =>
            Self::SouthWest =>
            Self::SouthEast =>
            Self::NorthWest =>
            Self::NorthEast =>,
            _ => false
        }
    }
}

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// struct Cell {
//     kind : CellType,
//     x : u32,
//     y : u32
// }

fn gen_adjacent(x:i32, y: i32, cell : CellType) -> Vec<(i32, i32)> {
    match cell {
        CellType::Vertical =>   vec![(x, y+1),(x, y-1)],
        CellType::Horizontal => vec![(x-1, y),(x+1, y)],
        CellType::NorthEast =>  vec![(x+1, y),(x, y+1)],
        CellType::NorthWest =>  vec![(x-1, y),(x, y+1)],
        CellType::SouthEast =>  vec![(x+1, y),(x, y-1)],
        CellType::SouthWest =>  vec![(x-1, y),(x, y-1)],
        CellType::None =>       vec![],
        _ => vec![(x-1, y),(x+1, y),(x, y+1),(x, y-1)]
    }
}

#[allow(dead_code)]
fn process1(input : &str) -> String {
    
    let map: HashMap<(i32, i32), CellType> = input
        .lines()
        .enumerate()
        .flat_map(|(idy, line)| {
            line.chars().enumerate().map(move |(idx, ch)| {
                ((idx.clone() as i32, idy.clone() as i32), CellType::from(ch))
            })
        })
        .filter(|el| {
            el.1 != CellType::None
        })
        .collect::<HashMap<(i32, i32), CellType>>();

    // Only one start point is in the map
    let start = map
        .iter()
        .find(|el| {*el.1 == CellType::Start})
        .unwrap();

    let mut cycle: VecDeque<(i32, i32)> = VecDeque::new();

    cycle.push_front(*start.0);

    loop {
        let last = *cycle.front().unwrap();
        dbg!(&cycle);
        
        for pos in gen_adjacent(last.0, last.1, *map.get(&last).unwrap()) {
            dbg!(&pos);
            // Filter last position
            if cycle.get(1).unwrap_or(&(-1,-1)) == &pos { continue; }

            match map.get(&pos) {
                Some(val) => {
                    // Validate if the cell connects

                    cycle.push_front(pos);
                    break;
                }
                None => {continue;}
            };
        }
        
        if *cycle.front().unwrap() == *start.0 {
            break;
        }
    }

    println!("{cycle:?}");

    // parse input
    // Find cycle
    
    (cycle.len() / 2).to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {
    _ = input;
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let exp_output = "4";
        assert_eq!(exp_output, process1(input));
    }

    #[ignore]
    #[test]
    fn test_process12() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let exp_output = "8";
        assert_eq!(exp_output, process1(input));
    }

    #[ignore]
    #[test]
    fn test_process2() {
        let input =  "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
        let exp_output = "2";
        assert_eq!(exp_output, process2(input));
    }

}