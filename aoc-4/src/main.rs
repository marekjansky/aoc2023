use std::collections::{BTreeSet, VecDeque};

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    println!("Result of computation: {output}");
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

#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
struct Card {
    id : u32,
    winning : BTreeSet<u32>,
    drawn : BTreeSet<u32>
}

impl Card {
    fn intersects(&mut self) -> Vec<u32>{
        self.drawn.intersection(&self.winning).cloned().collect()
    }

    fn from_line(line: &str) -> Card {
        let card_id = line.split(":")
            .next()
            .expect("Here is no last element")
            .split(" ").last().expect("No card ID number")
            .parse::<u32>()
            .expect("It is not a correct type");
        let card_game = line.split(":")
            .last()
            .expect("Here is no last element")
            .split("|")
            .collect::<Vec<_>>();

        Card {
            id: card_id,
            winning: card_game.first().unwrap().trim().split(" ").map(|x |{
                match x.parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => 0
                }
            }).filter(|x|{
                *x != 0
            }).collect(),  
            drawn: card_game.last().unwrap().trim().split(" ").map(|x |{
                match x.parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => 0
                }
            }).filter(|x|{
                *x != 0
            }).collect()
        }
    }
}

#[allow(dead_code)]
fn process2(input : &str) -> String {
    
    let games_orig: Vec<Card> = input.lines().map(|line|{
        Card::from_line(line)
    }).collect();

    let mut games = VecDeque::from(games_orig.clone());
    let mut card_counts: Vec<i64> = vec![0;games.len()];

    // dbg!(&games);
    // for (idx, mut game) in games {
    while games.len() > 0 {

        let mut card = games.pop_front().expect("There should be at least one element");
        
        card_counts[(card.id-1) as usize] += 1;
            
        let intersections = card.intersects();
        let no_cards = intersections.len() as u64;
        // println!("new cards: {}, card_id = {}", no_cards, card.id);
        // println!("intersect: {intersections:?}");

        for new_offsets in 0..no_cards {
            // dbg!(&new_id);
            let new_id = new_offsets + card.id as u64 + 1;
            let new_game = games_orig[(new_id-1) as usize].clone();
            // dbg!(&new_game);
            games.push_back(new_game);
        }
        // sleep(Duration::from_millis(500));
    }

    card_counts.iter().sum::<i64>().to_string()

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

    #[test]
    fn test_process2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let exp_output = "30";
        assert_eq!(exp_output, process2(input));
    }

}