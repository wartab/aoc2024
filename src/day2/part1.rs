use crate::day2::INPUT;

#[derive(PartialEq)]
enum Sign {
    Unknown,
    Positive,
    Negative,
}

pub fn part1() {
    let mut safe_count = 0usize;

    INPUT.split("\n").for_each(|line| {
        let mut last = -1i32;
        let mut safe = true;
        let mut expected_sign = Sign::Unknown;

        for x in line.split(' ').map(|x| x.parse::<i32>().unwrap()) {
            if last == -1 {
                last = x;
                continue;
            }

            let diff = x - last;

            if diff == 0 || diff > 3 || diff < -3 {
                safe = false;
                break;
            }

            let current_sign = if diff > 0 {
                Sign::Positive
            } else {
                Sign::Negative
            };

            if expected_sign == Sign::Unknown {
                expected_sign = current_sign;
            } else if expected_sign != current_sign {
                safe = false;
                break;
            }

            last = x;
        }

        if safe {
            println!("Safe line: {line}");
            safe_count += 1;
        } else {
            println!("Unsafe line: {line}");
        }
    });

    println!("Safe count: {safe_count}");
}