use std::{collections::HashMap, cell::Cell};

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

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let mut game_sum = 0;

    let mut schematic_map: HashMap<(i32, i32), CellType> = HashMap::new();
    
    let y_dim = input.lines().count() as i32;
    let x_dim = input.lines().next().unwrap().len() as i32;

    let mut chars : Vec<Vec<char>> = Vec::new();

    for (iy, line) in input.lines().enumerate(){
        let mut row: Vec<char> = Vec::new();

        for (ix, char) in line.chars().enumerate(){
            let idx : i32 = ix.try_into().unwrap();
            let idy : i32 = iy.try_into().unwrap();
            row.push(char);

            if char.is_numeric() {
                schematic_map.insert((idy, idx), CellType::Number);
            } else if char == '.' {
                // Empty space                
                schematic_map.insert((idy, idx), CellType::Void);

            } else {
                // Schematic symbol
                schematic_map.insert((idy, idx), CellType::Symbol);
            }
        }
        
        chars.push(row);
    }

    let mut valid_numbers: Vec<String> = Vec::new();

    let mut test_indicies : Vec<(i32, i32)> = Vec::new();

    for dy in -1..1{
        for dx in -1..1{
            if dy != 0 && dx != 0 {
                test_indicies.push((dy, dx));
            }
        }
    }


    for ((iy, ix), cell) in &schematic_map {
        // println!("{ix}, {iy}: {cell:?}");
        if *cell == CellType::Symbol {
            for (dy, dx) in &test_indicies {
                let new_x = (*ix as i32) + dx;
                let new_y = (*iy as i32) + dy;
                if new_x < x_dim && new_y < y_dim && new_y >= 0 && new_x >= 0 {
                    if schematic_map.get(&(new_y, new_x)).unwrap() == &CellType::Number {
                        // jackpot
                        // collect all numbers
                        let mut number = String::from(chars[new_y as usize][new_x as usize]);
                        for index in new_x..x_dim {
                            let znak = chars[new_y as usize][index as usize];
                            if znak.is_numeric() {
                                number.push(znak);
                            } else {
                                break;
                            }
                        }
                        for index in new_x..0 {
                            let znak = chars[new_y as usize][index as usize];
                            if znak.is_numeric() {
                                number.insert(0,znak);
                            } else {
                                break;
                            }
                        }

                        println!("{number}");
                    }
                }
            }
        }
    }

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