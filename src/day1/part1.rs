use std::collections::BinaryHeap;
use crate::day1::INPUT;

pub fn part1() {

    let mut left: BinaryHeap<u32> = BinaryHeap::new();
    let mut right: BinaryHeap<u32> = BinaryHeap::new();

    INPUT.split("\n").for_each(|line| {
        let mut numbers = line.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap());
        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();

        left.push(a);
        right.push(b);
    });

    let mut sum = 0;

    while let Some(a) = left.pop() {
        let b = right.pop().unwrap();

        let diff = if a < b {
            b - a
        } else {
            a - b
        };

        sum = sum + diff;
    }

    println!("Sum: {}", sum);
}