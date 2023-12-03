use std::{collections::HashMap, cell::Cell, intrinsics::mir::Len};

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    dbg!(output);
}

#[derive(Debug)]
#[derive(PartialEq)]
enum CellType {
    Void,
    Number,
    Symbol
}

fn gen_surrounding(x:u32, y:u32, x_dim : u32, y_dim : u32) -> Vec<(u32, u32)> {
    let mut test_indicies : Vec<(u32, u32)> = Vec::new();

    for dy in -1..2{
        for dx in -1..2{
            if dy != 0 || dx != 0 {
                let new_x = (x as i32) + dx;
                let new_y = (y as i32) + dy;
                if new_x < x_dim as i32 && new_y < y_dim as i32 && new_y >= 0 && new_x >= 0 {
                    test_indicies.push((new_y as u32, new_x as u32));
                }
            }
        }
    }

    test_indicies
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let mut game_sum = 0;

    // let mut schematic_map: HashMap<(i32, i32), CellType> = HashMap::new();


    let y_dim = input.lines().count() as u32;
    let x_dim = input.lines().next().unwrap().len() as u32;
    
    let mut valid_numbers: Vec<String> = Vec::new();

    let mut chars : Vec<char> = Vec::new();

    for (iy, line) in input.lines().enumerate(){

        for (ix, char) in line.chars().enumerate(){
            chars.push(char);            
        }
    }

    for iy in 0..chars.len() {
        for ix in 0..chars.first().unwrap().len(){ 
            let idx = ix as u32;
            let idy = iy as u32;
            let character = chars[idy*x_dim+idx];
            if !char.is_numeric() || char == '.' {
                // is symbol
                //check surroundings
                for (y, x) in gen_surrounding(idx, idy, x_dim, y_dim) {
                    input.lines().
                }
            }
        }        
    }


    dbg!(test_indicies);


    
    game_sum.to_string()
}


#[allow(dead_code)]
fn process2(input : &str) -> String {
    
    let mut game_sum = 0;
    
    game_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let exp_output = "4361";
        assert_eq!(exp_output, process1(input));
    }

    #[ignore]
    #[test]
    fn test_process2() {
        let input = "";
        let exp_output = "2286";
        assert_eq!(exp_output, process2(input));
    }

}