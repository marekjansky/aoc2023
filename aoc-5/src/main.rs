use std::{convert::From};
use rayon::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    println!("Result of computation: {output}");
}

#[derive(Debug)]
struct RangeMap {
    dst_start: u32,
    src_start: u32,
    rng_len: u32
}

impl RangeMap {
    fn map(&self, input : u32) -> Option<u32>{
        let rng = self.src_start..(self.src_start+self.rng_len);
        if rng.contains(&input) {
            return Some(self.dst_start + (input - self.src_start));
        } else {
            return None
        }
    }
}

impl From<&str> for RangeMap {
    fn from(item: &str) -> Self {
        let mut data = item
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().expect("This should be parsable"));
        RangeMap { 
            dst_start: data.next().expect("This should be here"), 
            src_start: data.next().expect("This should be here"), 
            rng_len: data.next().expect("This should be here")
         }
    }
}

#[derive(Debug)]
struct Transform {
    transforms: Vec<RangeMap>
}

impl Transform {
    fn apply(&self, input: u32) -> u32 {
        let mut output = 0;

        for trans in self.transforms.iter(){
            match trans.map(input) {
                Some(x) => {
                    output = x;
                    break;
                },
                None => {
                    output = input;
                }
            }
        }
        output
    }
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let mut lines = input.lines();
    
    let seeds = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().expect("This should be parsable")).collect::<Vec<_>>();
    
    let transforms = lines.fold(Vec::new(), |mut acc, x| {
        if x.is_empty() || acc.is_empty() {
            acc.push(
                Transform{transforms:Vec::new()});
        }
        if !x.is_empty() && !x.contains(':'){
            acc.last_mut().unwrap().transforms.push(RangeMap::from(x));
        }
        acc
    });

    let output: Vec<u32> = seeds.iter().map(|seed| {
        let mut transfer_var = *seed;
        for trans in transforms.iter() {
            transfer_var = trans.apply(transfer_var);
        }
        println!("Seed {seed} finished.");
        transfer_var
    }).collect();

    output.iter().min().expect("The vector should have something").to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {

    let mut lines = input.lines();
    
    let seed_ranges : Vec<std::ops::Range<u32>>= lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().expect("This should be parsable"))
        .fold(Vec::new(), |mut acc: Vec<Vec<u32>>, x| {
            if acc.is_empty() || acc.last_mut().unwrap().len() >= 2 {
                acc.push(Vec::new());
            }
            acc.last_mut().unwrap().push(x);
            acc
        }).iter()
        .map(|range| {
            let start = range[0];
            let count = range[1];
            start..(start+count)
        }).collect();
    
    let mut seeds = Vec::new();

    for range in seed_ranges.into_iter() {
        for seed in range {
            seeds.push(seed);
        }
    }

    let transforms = lines.fold(Vec::new(), |mut acc, x| {
        if x.is_empty() || acc.is_empty() {
            acc.push(
                Transform{transforms:Vec::new()});
        }
        if !x.is_empty() && !x.contains(':'){
            acc.last_mut().unwrap().transforms.push(RangeMap::from(x));
        }
        acc
    });

    let output: Vec<u32> = seeds.par_iter().map(|seed| {
        let mut transfer_var = *seed;
        for trans in transforms.iter() {
            transfer_var = trans.apply(transfer_var);
        }
        transfer_var
    }).collect();

    output.iter().min().expect("The vector should have something").to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let exp_output = "35";
        assert_eq!(exp_output, process1(input));
    }

    #[test]
    fn test_process2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let exp_output = "46";
        assert_eq!(exp_output, process2(input));
    }

}