fn main() {
    let input = include_str!("./input.txt");
    let output = process2(input);
    dbg!(output);
}

fn _process1(input : &str) -> String {
    
    let mut num : u32 = 0;

    for line in input.split("\n"){
        let mut num_str : String = String::new();
        for char in line.chars(){
            if char.is_numeric(){
                num_str.push(char);
                break;
            }
        }

        for char in line.chars().rev(){
            if char.is_numeric(){
                num_str.push(char);
                break;
            }
        }

        num += num_str.parse::<u32>().unwrap();
    }

    num.to_string()
}

fn process2(input : &str) -> String {
    
    let mut num : u32 = 0;

    let digit_lookup = [
        "one",
        "two", 
        "three", 
        "four", 
        "five", 
        "six", 
        "seven", 
        "eight", 
        "nine"
    ];

    for line in input.split("\n"){

        let mut temp_str : String = String::new();
        let mut num_str : String = String::new();

        for char in line.chars(){
            if char.is_numeric(){
                num_str.push(char);
                break;
            } else {
                temp_str.push(char);
                for (idx, digit) in digit_lookup.iter().enumerate(){
                    if temp_str.contains(digit){
                        temp_str.clear();
                        num_str.push((idx+1).to_string().chars().next().unwrap());
                        break;
                    }
                }

                if temp_str.is_empty(){
                    break;
                }
            }
        }

        for char in line.chars().rev(){
            if char.is_numeric(){
                num_str.push(char);
                break;
            } else {
                temp_str.insert(0, char);

                for (idx, digit) in digit_lookup.iter().enumerate(){
                    if temp_str.contains(digit){
                        temp_str.clear();
                        num_str.push((idx+1).to_string().chars().next().unwrap());
                        break;
                    }
                }

                if temp_str.is_empty(){
                    break;
                }
            }
        }

        dbg!(&num_str);


        num += num_str.parse::<u32>().unwrap();
    }

    num.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let exp_output = "142";
        assert_eq!(exp_output, _process1(input));
    }

    #[test]
    fn test_process2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let exp_output = "281";
        assert_eq!(exp_output, process2(input));
    }

}