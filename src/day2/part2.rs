use crate::day2::INPUT2;

#[derive(PartialEq, Debug)]
enum Sign {
    Unknown,
    Positive,
    Negative,
}


fn check_line(line: &[i32]) -> bool {
    let mut last = -1i32;
    let mut expected_sign = Sign::Unknown;

    for x in line {
        if *x == -1 {
            continue;
        }

        if last == -1 {
            last = *x;
            continue;
        }

        let diff = *x - last;

        if diff == 0 || diff > 3 || diff < -3 {
            return false;
        }

        let current_sign = if diff > 0 {
            Sign::Positive
        } else {
            Sign::Negative
        };

        if expected_sign == Sign::Unknown {
            expected_sign = current_sign;
        } else if expected_sign != current_sign {
            return false;
        }

        last = *x;
    }

    true
}

pub fn part2() {
    let mut safe_count = 0usize;

    INPUT2.split("\n").for_each(|line| {

        let mut line_vec = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if check_line(&line_vec) {
            safe_count += 1;
        } else {
            for i in 0..line_vec.len() {
                let old = line_vec[i];
                line_vec[i] = -1;

                if check_line(&line_vec) {
                    safe_count += 1;
                    break;
                }

                line_vec[i] = old;
            }
        }
    });

    println!("Safe count: {safe_count}");
}