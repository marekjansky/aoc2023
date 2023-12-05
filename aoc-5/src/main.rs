use std::{convert::From, fmt::Error, sync::BarrierWaitResult};

fn main() {
    let input = include_str!("./input.txt");
    let output = process1(input);
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
        if input == 53 {
            dbg!(&rng);   
        }
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
        dbg!(output);
        output
    }
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let mut lines = input.lines();
    
    // let rng = RangeMap {
    //     dst_start: 42,
    //     src_start: 0,
    //     rng_len: 7,
    // };

    // let out = rng.map(49);

    // dbg!(out);

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
        transfer_var
    }).collect();

    output.iter().min().expect("The vector should have something").to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {
    for line in input.lines(){
        _ = line.trim();
    }
    "".to_string()

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

    #[ignore]
    #[test]
    fn test_process2() {
        let input = "";
        let exp_output = "30";
        assert_eq!(exp_output, process2(input));
    }

}