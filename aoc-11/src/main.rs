use std::collections::{HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process1(input);
    println!("Result of computation: {output}");
}

#[allow(dead_code)]
fn process1(input : &str) -> String {
    
    let galaxy: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(_idy, line)| {
            line.char_indices().map(|(_idx, ch)| {
                ch
            }).collect::<Vec<char>>()
        })
        .collect();

    
    let empty_rows : Vec<u32> = galaxy.iter().map(|line| {
            line.iter().any(|char| *char == '#')
        }).enumerate().filter(|(_idx, val)| !*val).map(|val| val.0 as u32).collect();

    let empty_cols : Vec<u32> = (0..galaxy[0].len()).map(|idx| {
            galaxy.iter().any(|line| { line[idx] == '#'})
        }).enumerate().filter(|(_idx, val)| !*val).map(|val| val.0 as u32).collect();

    let galaxies: Vec<(i32, i32)> = galaxy.iter().enumerate().flat_map(|(idy, line)|{
        line.iter().enumerate().filter_map(move|(idx, char)|{
            if *char == '#' {
                Some((idx as i32, idy as i32))
            } else {
                None
            }
        })
    }).collect();

    let edges: Vec<((i32, i32), (i32, i32))> = galaxies.iter().cloned().permutations(2).unique().map(|perm| (perm[0], perm[1])).collect::<Vec<_>>();
    
    let mut edges_filt: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    for (from, to) in &edges {
        if !edges_filt.contains(&(*to, *from)) && !edges_filt.contains(&(*from, *to)) {
            edges_filt.insert((*from, *to));
        }
    }

    let distances = edges_filt.iter().map(|(from, to)| {
        let steps_x = to.0.abs_diff(from.0);
        let steps_y = to.1.abs_diff(from.1);

        let x_start = from.0.min(to.0);
        let x_end = from.0.max(to.0);
        let y_start = from.1.min(to.1);
        let y_end = from.1.max(to.1);

        let space_x = empty_cols.iter().filter(|col| { 
            (**col as i32) > x_start && (**col as i32) < x_end
         }).count();

         let space_y = empty_rows.iter().filter(|row| { 
            (**row as i32) > y_start && (**row as i32) < y_end
         }).count();

        steps_x + steps_y + space_x as u32 + space_y as u32
    });

    distances.sum::<u32>().to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {
    
    let galaxy: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(_idy, line)| {
            line.char_indices().map(|(_idx, ch)| {
                ch
            }).collect::<Vec<char>>()
        })
        .collect();

    
    let empty_rows : Vec<u32> = galaxy.iter().map(|line| {
            line.iter().any(|char| *char == '#')
        }).enumerate().filter(|(_idx, val)| !*val).map(|val| val.0 as u32).collect();

    let empty_cols : Vec<u32> = (0..galaxy[0].len()).map(|idx| {
            galaxy.iter().any(|line| { line[idx] == '#'})
        }).enumerate().filter(|(_idx, val)| !*val).map(|val| val.0 as u32).collect();

    let galaxies: Vec<(i32, i32)> = galaxy.iter().enumerate().flat_map(|(idy, line)|{
        line.iter().enumerate().filter_map(move|(idx, char)|{
            if *char == '#' {
                Some((idx as i32, idy as i32))
            } else {
                None
            }
        })
    }).collect();

    let edges: Vec<((i32, i32), (i32, i32))> = galaxies.iter().cloned().permutations(2).unique().map(|perm| (perm[0], perm[1])).collect::<Vec<_>>();
    
    let mut edges_filt: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    for (from, to) in &edges {
        if !edges_filt.contains(&(*to, *from)) && !edges_filt.contains(&(*from, *to)) {
            edges_filt.insert((*from, *to));
        }
    }

    let distances = edges_filt.iter().map(|(from, to)| {
        let steps_x = to.0.abs_diff(from.0);
        let steps_y = to.1.abs_diff(from.1);

        let x_start = from.0.min(to.0);
        let x_end = from.0.max(to.0);
        let y_start = from.1.min(to.1);
        let y_end = from.1.max(to.1);

        let space_x = empty_cols.iter().filter(|col| { 
            (**col as i32) > x_start && (**col as i32) < x_end
         }).count() * 10;

         let space_y = empty_rows.iter().filter(|row| { 
            (**row as i32) > y_start && (**row as i32) < y_end
         }).count() * 10;

        steps_x + steps_y + space_x as u32 + space_y as u32
    });

    distances.sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let exp_output = "374";
        assert_eq!(exp_output, process1(input));
    }
    #[test]
    fn test_process2() {
        let input = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let exp_output = "1030";
        assert_eq!(exp_output, process2(input));
    }



}