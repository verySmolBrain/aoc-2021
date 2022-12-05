use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> u32 {
    let line_length = input.lines().next().unwrap().len();
    let input_length = input.lines().count();

    let mut gamma: String = String::from("");
    let mut epsilon: String = String::from("");
    
    for i in 0..line_length {
        let num_ones = input
            .lines()
            .map(|line| line.chars().nth(i).unwrap())
            .fold(0, |acc, c| c.to_digit(10).unwrap() + acc);
        
        if num_ones as usize > input_length / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            epsilon.push('1');
            gamma.push('0');
        }
    }
    
    return u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap();
}

fn calculate(input: &str, f: impl Fn(&str, u32, usize) -> String) -> i32 {
    let line_length = input.lines().next().unwrap().len();
    let mut oxy: String = String::from("");

    for i in 0..line_length {
        let oxy_ones = input
            .lines()
            .filter(|l| l.starts_with(&oxy))
            .map(|line| line.chars().nth(i).unwrap())
            .fold(0, |acc, c| c.to_digit(10).unwrap() + acc);

        let length = input
            .lines()
            .filter(|l| l.starts_with(&oxy))
            .count();
        
        if length == 1 {
            let binary = input
                .lines()
                .filter(|l| l.starts_with(&oxy))
                .next()
                .unwrap();
            
            return i32::from_str_radix(&binary, 2).unwrap();
        }
        
        oxy = f(&oxy, oxy_ones, length);
    }

    return i32::from_str_radix(&oxy, 2).unwrap();
}

fn part2(input: &str) -> i32 {
    let oxy_push = |oxy: &str, ones: u32, length: usize| -> String {
        let mut oxy: String = oxy.to_string();

        if ones * 2 >= length as u32 {
            oxy.push('1');
        } else {
            oxy.push('0');
        }

        return oxy;
    };

    let co2_push = |co2: &str, ones: u32, length: usize| -> String {
        let mut co2: String = co2.to_string();

        if ones * 2 >= length as u32 {
            co2.push('0');
        } else {
            co2.push('1');
        }

        return co2;
    };

    let oxy: i32 = calculate(input, oxy_push) as i32;
    let co2: i32 = calculate(input, co2_push) as i32;

    return oxy * co2;
}