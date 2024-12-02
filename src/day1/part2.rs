use crate::day1::INPUT;
use std::collections::HashMap;

pub fn part2() {
    let mut left = vec![0u32; 1024];
    let mut right: HashMap<u32, u32> = HashMap::new();

    INPUT.split("\n").for_each(|line| {
        let mut numbers = line.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap());
        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();

        left.push(a);
        right.entry(b).and_modify(|e| *e += 1).or_insert(1);
    });

    let mut sum = 0;

    left.iter().for_each(|a| {
        let b = right.get(a);

        if let Some(b) = b {
            sum = sum + a * b;
        }

    });

    println!("Sum: {sum}");
}