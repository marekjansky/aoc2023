use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    dbg!(output);
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let max_counts: HashMap<&str, i32> = HashMap::from([
        ("blue", 14),
        ("green", 13),
        ("red", 12),
    ]);

    let mut game_sum = 0;
    
    for line in input.lines(){

        let mut tmp = line.split(":");
        let game_id : u32 = tmp.next().unwrap().split(" ").nth(1).unwrap().parse().unwrap();
        let mut valid = true;
        for cubes in tmp.next().unwrap().trim().split(";"){
            for draw in cubes.split(","){
                let mut cube = draw.trim().split(" ");
                let cube_cnt : i32 = cube.next().unwrap().parse().unwrap();
                let cube_color = cube.next().unwrap().to_string();
                let color_max_count = **&max_counts.get(&cube_color.as_str()).unwrap();
                println!("Max cubes {color_max_count} for color {cube_color}");

                if color_max_count < cube_cnt {
                    valid = false;
                }
            }
        }
        if valid == true {
            game_sum += game_id;
        }

        // println!("ID: {game_id}, SUM: {game_sum}");

    }
    game_sum.to_string()
}


#[allow(dead_code)]
fn process2(input : &str) -> String {
    
    let mut game_sum = 0;
    
    for line in input.lines(){

        let mut tmp = line.split(":");
        let game_id : u32 = tmp.next().unwrap().split(" ").nth(1).unwrap().parse().unwrap();

        let mut min_color_counts: HashMap<&str, i32> = HashMap::from([
            ("blue", 0),
            ("green", 0),
            ("red", 0),
        ]);

        for cubes in tmp.next().unwrap().trim().split(";"){
            for draw in cubes.split(","){
                let mut cube = draw.trim().split(" ");
                let cube_cnt : i32 = cube.next().unwrap().parse().unwrap();
                let cube_color = cube.next().unwrap();
                let min_count = min_color_counts.get(&cube_color).unwrap();

                if *min_count < cube_cnt {
                    min_color_counts.insert(cube_color, cube_cnt);
                }
                
            }
        }

        let min_counts : i32 = min_color_counts.values().product();
        game_sum += min_counts;
        println!("ID: {game_id}, SUM: {game_sum}");

    }
    game_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let exp_output = "8";
        assert_eq!(exp_output, process1(input));
    }

    // #[ignore]
    #[test]
    fn test_process2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let exp_output = "2286";
        assert_eq!(exp_output, process2(input));
    }

}