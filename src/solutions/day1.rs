use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut prev: i32;
    let mut cur: i32 = i32::MAX;
    let mut num_increases: i32 = 0;

    for line in input.lines() {
        let num = line.parse::<i32>().unwrap();

        prev = cur;
        cur = num;

        if cur > prev {
            num_increases += 1;
        }
    }

    return num_increases;
}

fn part2(input: &str) -> i32 {
    let mut win: Vec<i32> = Vec::new();
    let win_size = 3;

    let mut prev: i32;
    let mut cur: i32 = i32::MAX;
    let mut num_increases: i32 = 0;

    for line in input.lines() {
        let num = line.parse::<i32>().unwrap();
        win.push(num);

        if win.len() > win_size {
            win.remove(0);
        } else {
            continue;
        }

        let sum = win.iter().sum::<i32>();

        prev = cur;
        cur = sum;

        if cur > prev {
            num_increases += 1;
        }
    }

    return num_increases;
}