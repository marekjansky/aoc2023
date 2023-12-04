use std::collections::BTreeSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process1(input);
    dbg!(output);
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let game_sum: u32 = input.lines().map(|line|{
        let game = line.split(":").last().expect("Here is no last element");
        let mut game_numbers = game.split("|");
        let winning : BTreeSet<u32> = game_numbers.next().unwrap().trim().split(" ").map(|x |{
            match x.parse::<u32>() {
                Ok(x) => x,
                Err(_) => 0
            }
            
        }).filter(|x|{
            *x != 0
        }).collect();
        
        let drawn : BTreeSet<u32> = game_numbers.last().unwrap().trim().split(" ").map(|x |{
            match x.parse::<u32>() {
                Ok(x) => x,
                Err(_) => 0
            }
        }).filter(|x|{
            *x != 0
        }).collect();

        let no_cards = drawn.intersection(&winning).cloned().count();
        match no_cards {
            x if x > 0 => 2_u32.pow((x-1).try_into().unwrap()),
            _ => 0
        }
    }).sum();
    
    game_sum.to_string()
}


#[allow(dead_code)]
fn process2(input : &str) -> String {
    
    let game_sum = 0;
    
    game_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let exp_output = "13";
        assert_eq!(exp_output, process1(input));
    }

    #[ignore]
    #[test]
    fn test_process2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let exp_output = "2286";
        assert_eq!(exp_output, process2(input));
    }

}