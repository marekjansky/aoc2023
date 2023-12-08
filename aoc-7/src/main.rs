use std::{collections::HashMap,cmp::Ordering};
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    println!("Result of computation: {output}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

fn score_hand(cards: &str) -> HandType {

    let char_counts = cards
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, i32>, value|{
            *acc.entry(value).or_insert(0) += 1;
            acc
    });    
        
    match char_counts.len() {
        1 => HandType::FiveOfaKind,
        2 => {
            if char_counts.values().any(|val| *val == 4) {
                HandType::FourOfaKind
            } else {
                HandType::FullHouse
            }
        },
        3 => {
            if char_counts.values().any(|val| *val == 3) {
                HandType::ThreeOfaKind
            } else {
                HandType::TwoPair
            }
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!()
    }
}

fn cmp_hands(hand1:&(&str, u32, HandType), hand2:&(&str, u32, HandType)) -> Ordering {
        
    let cards_lut: HashMap<char, i32> =
        [('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 11),
        ('K', 12),
        ('A', 13)]
        .iter().cloned().collect();


    if hand1.2 == hand2.2 {
        for (char1, char2) in hand1.0.chars().zip(hand2.0.chars()){
            let ch1score = *cards_lut.get(&char1).unwrap();
            let ch2score = *cards_lut.get(&char2).unwrap();
            if ch1score == ch2score {
                continue;
            } else {
                return ch1score.cmp(&ch2score)
            }
        }
        Ordering::Equal
    } else {
        hand1.2.cmp(&hand2.2)
    }
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let lines = input
        .lines();

    let mut hands = lines
        .map(|line|{
            let cards = line.split_ascii_whitespace().nth(0).unwrap();
            let bid: u32 = line.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
            (cards, bid)
        })
        .map(|(cards, bid)| {
            (cards, bid, score_hand(cards))
        })
        .collect::<Vec<_>>();

    hands.sort_by(|game1, game2| {
        cmp_hands(game1, game2)
        // game1.1.cmp(&game2.1)
    });

    let score = hands
        .iter()
        .enumerate()
        .fold(0u32, |acc, (idx, val)| {
            acc + val.1 * (idx as u32 + 1)
    });

    dbg!(&hands);

    score.to_string()
}

fn score_hand2(cards: &str) -> HandType {

    let mut char_counts = cards.chars().counts();
    
    dbg!(&char_counts);

    let j_score = match char_counts.remove(&'J') {
        Some(x) => x,
        None => 0
    };

    if char_counts.is_empty(){
        char_counts.insert('A', 5);
    } else {
        let max_el = char_counts.iter().max_by(|el1, el2|{
            el1.1.cmp(el2.1)
        }).unwrap().0.clone();
    
        char_counts.entry(max_el).and_modify(|element| *element += j_score);
    }
    
    match char_counts.len() {
        1 => HandType::FiveOfaKind,
        2 => {
            if char_counts.values().any(|val| *val == 4) {
                HandType::FourOfaKind
            } else {
                HandType::FullHouse
            }
        },
        3 => {
            if char_counts.values().any(|val| *val == 3) {
                HandType::ThreeOfaKind
            } else {
                HandType::TwoPair
            }
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!()
    }
}

#[allow(dead_code)]
fn process2(input : &str) -> String {
    
    let lines = input
        .lines();

    let mut hands = lines
        .map(|line|{
            let cards = line.split_ascii_whitespace().nth(0).unwrap();
            let bid: u32 = line.split_ascii_whitespace().nth(1).unwrap().parse().unwrap();
            (cards, bid)
        })
        .map(|(cards, bid)| {
            (cards, bid, score_hand2(cards))
        })
        .collect::<Vec<_>>();

    hands.sort_by(|game1, game2| {
        cmp_hands(game1, game2)
    });

    let score = hands
        .iter()
        .enumerate()
        .fold(0u32, |acc, (idx, val)| {
            acc + val.1 * (idx as u32 + 1)
    });

    dbg!(&hands);

    score.to_string()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let exp_output = "6440";
        assert_eq!(exp_output, process1(input));
    }

    #[test]
    fn test_process2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let exp_output = "5905";
        assert_eq!(exp_output, process2(input));
    }

}