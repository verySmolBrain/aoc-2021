use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let command: &str = split.next().unwrap();
        let value: i32 = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => unreachable!(),
        }
    }

    return horizontal * depth;
}

fn part2(input: &str) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let command: &str = split.next().unwrap();
        let value: i32 = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }

    return horizontal * depth;
}