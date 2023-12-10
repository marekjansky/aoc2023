use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
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
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'J' => Self::NorthWest,
            'L' => Self::NorthEast,
            _ => Self::None
        }
    }
}

// impl CellType {
//     fn connects(&self, other: &CellType, relative_position : Position) -> bool {
//         match self {
//             Self::Horizontal  
//             Self::Vertical =>
//             Self::SouthWest =>
//             Self::SouthEast =>
//             Self::NorthWest =>
//             Self::NorthEast =>,
//             _ => false
//         }
//     }
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// struct Cell {
//     kind : CellType,
//     x : u32,
//     y : u32
// }

fn gen_adjacent(cell : CellType) -> Vec<Position> {
    // match cell {
    //     CellType::Vertical =>   vec![(x, y+1),(x, y-1)],
    //     CellType::Horizontal => vec![(x-1, y),(x+1, y)],
    //     CellType::NorthEast =>  vec![(x+1, y),(x, y+1)],
    //     CellType::NorthWest =>  vec![(x-1, y),(x, y+1)],
    //     CellType::SouthEast =>  vec![(x+1, y),(x, y-1)],
    //     CellType::SouthWest =>  vec![(x-1, y),(x, y-1)],
    //     CellType::None =>       vec![],
    //     _ => vec![(x-1, y),(x+1, y),(x, y+1),(x, y-1)]
    // }
    use Position::*;

    match cell {
        CellType::Vertical =>   vec![North, South],
        CellType::Horizontal => vec![West,  East],
        CellType::NorthEast =>  vec![North, East],
        CellType::NorthWest =>  vec![North, West],
        CellType::SouthEast =>  vec![South, East],
        CellType::SouthWest =>  vec![South, West],
        CellType::None =>       vec![],
        _ => vec![South, North, West, East]
    }
}

// fn gen_possible(cell : CellType) -> Vec<CellType> {
//     match cell {
//         CellType::Vertical =>   vec![CellType::Vertical, CellType::No],
//         CellType::Horizontal => vec![(x-1, y),(x+1, y)],
//         CellType::NorthEast =>  vec![(x+1, y),(x, y+1)],
//         CellType::NorthWest =>  vec![(x-1, y),(x, y+1)],
//         CellType::SouthEast =>  vec![(x+1, y),(x, y-1)],
//         CellType::SouthWest =>  vec![(x-1, y),(x, y-1)],
//         CellType::None =>       vec![],
//         _ => vec![(x-1, y),(x+1, y),(x, y+1),(x, y-1)]
//     }
// }

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

    let mut graph: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    // Generate graph
    for ((posx, posy), kind) in map.iter() {
        for new_pos in gen_adjacent(*kind) {
            let new_idx = match new_pos {
                Position::East => (posx + 1, *posy),
                Position::West => (posx - 1, *posy),
                Position::North => (*posx, posy - 1),
                Position::South => (*posx, posy + 1)
            };

            if map.contains_key(&new_idx) {
                graph.entry((*posx, *posy)).or_insert(Vec::new()).push(new_idx);
            }
        }
    }

    // dbg!(&graph);

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut to_visit: VecDeque<(i32, i32)> = VecDeque::new();
    to_visit.push_front(*start.0);

    while !to_visit.is_empty() {
        let visiting: (i32, i32) = to_visit.pop_front().unwrap();

        let links = graph.get(&visiting).unwrap();
        
        if visited.contains(&visiting) {
            // println!("Already visited this");
            break;
        } else {
            visited.push(visiting);
        }

        for link in links {
            if !visited.contains(link) {
                to_visit.push_front(*link);
            }
        }
    }

    
    (visited.len() / 2).to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {
    
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

    let mut graph: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    // Generate graph
    for ((posx, posy), kind) in map.iter() {
        for new_pos in gen_adjacent(*kind) {
            let new_idx = match new_pos {
                Position::East => (posx + 1, *posy),
                Position::West => (posx - 1, *posy),
                Position::North => (*posx, posy - 1),
                Position::South => (*posx, posy + 1)
            };

            if map.contains_key(&new_idx) {
                graph.entry((*posx, *posy)).or_insert(Vec::new()).push(new_idx);
            }
        }
    }

    // dbg!(&graph);

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut to_visit: VecDeque<(i32, i32)> = VecDeque::new();
    to_visit.push_front(*start.0);

    while !to_visit.is_empty() {
        let visiting: (i32, i32) = to_visit.pop_front().unwrap();

        let links = graph.get(&visiting).unwrap();
        
        if visited.contains(&visiting) {
            // println!("Already visited this");
            break;
        } else {
            visited.push(visiting);
        }

        for link in links {
            if !visited.contains(link) {
                to_visit.push_front(*link);
            }
        }
    }

    // Cycle is stored in visited
    // Now we need to calculate the area that the cycle encloses

    let min_x = *visited.iter().map(|(x, _y)|{
        x
    }).min().unwrap();

    let max_x = *visited.iter().map(|(x, _y)|{
        x
    }).max().unwrap();

    let min_y = *visited.iter().map(|(_x, y)|{
        y
    }).min().unwrap();

    let max_y = *visited.iter().map(|(_x, y)|{
        y
    }).max().unwrap();

    // let x_size = max_x - min_x;
    // let y_size = max_y - min_y;

    // let visited_set : HashSet<(i32, i32)> = visited.iter().cloned().collect();

    // let point_map = (min_y..=max_y)
    //     .flat_map(|y| {
    //         (min_x..=max_x).map(move |x|{
    //             (x, y)
    //         })
    //     })
    //     .collect::<HashSet<_>>();

    // let points_to_explore = point_map.difference(&visited_set);
    
    // for point in visited {
    //     let cell_type = map.get(&point).unwrap();

    // }

    let mut count = 0;

    for idy in min_y..=max_y {
        let mut crossings = 0;
        for idx in min_x..=max_x {
            if visited.contains(&(idx, idy)) {
                match map.get(&(idx, idy)).unwrap() {
                    CellType::Vertical => {
                        crossings += 1;
                    },
                    CellType::Start => {
                        crossings += 1;
                    },
                    CellType::SouthEast => {
                        crossings += 1;
                    },
                    CellType::SouthWest => {
                        crossings += 1;
                    },
                    _ => {}
                }

            } else {
                if crossings % 2 != 0 {
                    count += 1;
                }
            }
        }
    }


    
    (count).to_string()
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

    #[test]
    fn test_process2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let exp_output = "10";
        assert_eq!(exp_output, process2(input));
    }

    #[test]
    fn test_process22() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let exp_output = "8";
        assert_eq!(exp_output, process2(input));
    }

    #[test]
    fn test_process23() {
        let input = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let exp_output = "4";
        assert_eq!(exp_output, process2(input));
    }



}