use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> Vec<i64> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let numbers: Vec<i64> = lines
        .filter(|x| x.as_ref().unwrap() != "")
        .map(|x| i64::from_str(&x.unwrap()).unwrap())
        .collect();
    numbers
}

fn find_weakness(numbers: Vec<i64>) -> (usize, i64) {
    let mut index = 25;
    'a: while index < numbers.len() {
        let num = numbers[index];
        for i in index - 25..index {
            let must_match = num - numbers[i];
            for j in i..index {
                if must_match == numbers[j] {
                    index += 1;
                    continue 'a;
                }
            }
        }
        return (index, num);
    }
    (0, 0)
}

fn exploit_weakness(numbers: Vec<i64>, number: i64) -> i64 {
    'a: for i in 0..numbers.len() {
        let mut possibles = vec![numbers[i]];
        'b: for j in i + 1..numbers.len() {
            possibles.push(numbers[j]);
            match possibles.iter().sum::<i64>() {
                sum if sum < number => continue 'b,
                sum if sum > number => continue 'a,
                _ => return possibles.iter().min().unwrap() + possibles.iter().max().unwrap(),
            }
        }
    }
    return 0
} 

fn main() {
    let numbers = parse_input();
    let (_, number) = find_weakness(numbers.clone());
    let exploit = exploit_weakness(numbers, number);
    println!("exploit: {}", exploit);
}