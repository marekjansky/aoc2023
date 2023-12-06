use core::time;


fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    println!("Result of computation: {output}");
}

#[allow(dead_code)]
fn process1(input : &str) -> String {

    let lines = input.lines();

    let mut lines_parsed = lines
    .map(|line| {
        line
        .split_ascii_whitespace()
        .skip(1)
        .map(|num| {
            num.parse().unwrap()
        })
        .collect::<Vec<u32>>()
    });

    let times = lines_parsed.next().unwrap();
    let distance = lines_parsed.next().unwrap();
    let res = times
        .iter()
        .zip(distance.iter())
        .map(|(max_time, distance)| {
            (0..=*max_time)
                .map(move |speed| {
                    let remaining_time = max_time - speed;
                    let cur_dist = remaining_time * speed;
                    cur_dist > *distance
                })
                .filter(|val| *val)
                .count()
            
        }).reduce(|acc, val| acc * val).unwrap() as u32;
    

    res.to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {

    
    let lines = input.lines();

    let mut lines_parsed = lines
    .map(|line| {
        line
        .split_ascii_whitespace()
        .skip(1)
        .fold(String::new(), |acc, val| acc + val)
        .parse::<u64>().unwrap()
    });

    let max_time = lines_parsed.next().unwrap();
    let distance = lines_parsed.next().unwrap();

    let res = (0..=max_time)
        .map(move |speed| {
            let remaining_time = max_time - speed;
            let cur_dist = remaining_time * speed;
            cur_dist > distance
        })
        .filter(|val| *val)
        .count();
               

    res.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let exp_output = "288";
        assert_eq!(exp_output, process1(input));
    }

    #[test]
    fn test_process2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let exp_output = "71503";
        assert_eq!(exp_output, process2(input));
    }

}