use std::collections::VecDeque;


fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    println!("Result of computation: {output}");
}

fn vec_diff(vec : &Vec<i32>) -> Vec<i32> {
    let (_last, vec1) = vec.split_last().unwrap();
    let (_first, vec2) = vec.split_first().unwrap();

    vec1.iter().zip(vec2.iter()).map(|(el1, el2)|{ el2 - el1}).collect::<Vec<_>>()

}

#[allow(dead_code)]
fn process1(input : &str) -> String {
    
    let lines = input
        .lines();

    let predictions: i32 = lines
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            
            let mut diff: VecDeque<Vec<i32>> = VecDeque::new();
            diff.push_front(numbers);

            while diff.front().unwrap().iter().filter(|val| val != &&0).count() != 0 {
                let diference = vec_diff(diff.front().unwrap());
                diff.push_front(diference);
            }

            let mut val_to_add = 0;
            for diference in diff.split_off(1) {
                // dbg!(&diference);
                val_to_add = diference.last().unwrap() + val_to_add;
            }

            val_to_add
        })
        .sum()
        ;

    predictions.to_string()
}

#[allow(dead_code)]
fn process2(input : &str) -> String {

    
    let lines = input
        .lines();

    let predictions: i32 = lines
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            
            let mut diff: VecDeque<Vec<i32>> = VecDeque::new();
            diff.push_front(numbers);

            while diff.front().unwrap().iter().filter(|val| val != &&0).count() != 0 {
                let diference = vec_diff(diff.front().unwrap());
                diff.push_front(diference);
            }

            let mut val_to_add = 0;
            for diference in diff.split_off(1) {
                // dbg!(&diference);
                val_to_add = diference.first().unwrap() - val_to_add;
            }
            dbg!(&val_to_add);
            val_to_add
        })
        .sum()
        ;

    predictions.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let exp_output = "114";
        assert_eq!(exp_output, process1(input));
    }

    #[test]
    fn test_process2() {
        let input =  "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
        let exp_output = "2";
        assert_eq!(exp_output, process2(input));
    }

}