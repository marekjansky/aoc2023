use std::collections::{BTreeMap, HashMap};
use num::integer::lcm;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    println!("Result of computation: {output}");
}

#[allow(dead_code)]
fn process1(input : &str) -> String {
    
    let mut lines = input
        .lines();

    let instr = lines.next().expect("Should be here").chars().cycle();
    _ = lines.next();

    let map = lines.map(|line| {
        let mut parsedline = line.split(" = ");
        let entry = parsedline.next().unwrap();
        let paths = parsedline
            .next()
            .unwrap()
            .trim_matches(|x| { x == ')' || x== '('})
            .split(", ")
            .map(|val| val.to_string())
            .collect_tuple::<(String, String)>().unwrap();
        (entry.to_string(), paths)
    }).collect::<BTreeMap<String, (String, String)>>();

    let mut current_pos = map.first_key_value().unwrap();
    let mut count: u32 = 0;
    for instruction in instr {
        if *current_pos.0 == "ZZZ".to_string() {
            break;
        }
        
        match instruction {
            'L' => current_pos = map.get_key_value(&current_pos.1.0).unwrap(),
            'R' => current_pos = map.get_key_value(&current_pos.1.1).unwrap(),
            _ => panic!("Never should get here")
        }

        count += 1;
    }



    count.to_string()

}

#[allow(dead_code)]
fn process2(input : &str) -> String {

    let mut lines = input
        .lines();

    let instr = lines.next().expect("Should be here").chars().cycle();
    _ = lines.next();

    let map = lines.map(|line| {
        let mut parsedline = line.split(" = ");
        let entry = parsedline.next().unwrap();
        let paths = parsedline
            .next()
            .unwrap()
            .trim_matches(|x| { x == ')' || x== '('})
            .split(", ")
            .map(|val| val.to_string())
            .collect_tuple::<(String, String)>().unwrap();
        (entry.to_string(), paths)
    }).collect::<HashMap<String, (String, String)>>();

    let mut positions = map.iter().filter(|(key, _val)| {
        key.ends_with('A')
    }).collect_vec();

    // dbg!(&positions);
    // dbg!(&positions.len());

    // let mut current_pos = map.first_key_value().unwrap();
    let mut count: Vec<i32> = Vec::new();

    for (_idx, current_pos) in positions.iter_mut().enumerate() {
        count.push(0);
        for instruction in instr.clone() {
            if current_pos.0.ends_with('Z') { break;}

            match instruction {
                'L' => *current_pos = map.get_key_value(&current_pos.1.0).unwrap(),
                'R' => *current_pos = map.get_key_value(&current_pos.1.1).unwrap(),
                _ => panic!("Never should get here")
            }
            *count.last_mut().unwrap() += 1;
        }        
        // dbg!(&positions);

        // let ending_positions = positions.iter().filter(|val| {
        //     val.0.ends_with('Z')
        // }).count();
        
        // dbg!(&ending_positions);

        // if ending_positions == 2 {
        //     break;
        // }


    }

    let lcm_val = count.iter().fold(1i64, |acc, num| {
        lcm(acc, *num as i64)
    });


    lcm_val.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let exp_output = "2";
        assert_eq!(exp_output, process1(input));
    }

    #[test]
    fn test_process12() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let exp_output = "6";
        assert_eq!(exp_output, process1(input));
    }

    #[test]
    fn test_process2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let exp_output = "6";
        assert_eq!(exp_output, process2(input));
        panic!();
    }

}