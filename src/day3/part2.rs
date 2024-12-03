use crate::day3::INPUT;

#[derive(Debug)]
enum State {
    Text,
    MulOperator(usize),
    CondOperator(&'static str),
    OpenParen,
    FirstNumber(u32),
    Comma,
    SecondNumber(u32),
}

pub fn part2() {
    let input = INPUT;

    let mut result = 0;

    let mut skipping = false;
    let mut state = State::Text;
    let mut first_number = 0u32;

    input.chars().for_each(|c| {
        match state {
            State::Text if c == 'd' => {
                state = State::CondOperator("d");
            }
            State::CondOperator(prefix) if prefix == "d" && c == 'o' => {
                state = State::CondOperator("do");
            }
            State::CondOperator(prefix) if prefix == "do" && c == '(' => {
                state = State::CondOperator("do(");
            }
            State::CondOperator(prefix) if prefix == "do(" && c == ')' => {
                skipping = false;
                state = State::Text;
            }
            State::CondOperator(prefix) if prefix == "do" && c == 'n' => {
                state = State::CondOperator("don");
            }
            State::CondOperator(prefix) if prefix == "don" && c == '\'' => {
                state = State::CondOperator("don'");
            }
            State::CondOperator(prefix) if prefix == "don'" && c == 't' => {
                state = State::CondOperator("don't");
            }
            State::CondOperator(prefix) if prefix == "don't" && c == '(' => {
                state = State::CondOperator("don't(");
            }
            State::CondOperator(prefix) if prefix == "don't(" && c == ')' => {
                skipping = true;
                state = State::Text;
            }
            State::Text if !skipping && c == 'm' => {
                state = State::MulOperator(0);
            }
            State::MulOperator(index) if index == 0 && c == 'u' => {
                state = State::MulOperator(1);
            }
            State::MulOperator(index) if index == 1 && c == 'l' => {
                state = State::MulOperator(2);
            }
            State::MulOperator(index) if index == 2 && c == '(' => {
                state = State::OpenParen;
            }
            State::OpenParen if c >= '1' && c <= '9' => {
                state = State::FirstNumber(c.to_digit(10).unwrap());
            }
            State::FirstNumber(prefix) if c >= '0' && c <= '9' && prefix < 100 => {
                let digit = c.to_digit(10).unwrap();
                state = State::FirstNumber(prefix * 10 + digit);
            }
            State::FirstNumber(prefix) if c == ',' => {
                first_number = prefix;
                state = State::Comma;
            }
            State::Comma if c >= '1' && c <= '9' => {
                state = State::SecondNumber(c.to_digit(10).unwrap());
            }
            State::SecondNumber(prefix) if c >= '0' && c <= '9' && prefix < 100 => {
                let digit = c.to_digit(10).unwrap();
                state = State::SecondNumber(prefix * 10 + digit);
            }
            State::SecondNumber(prefix) if c == ')' => {
                result += first_number * prefix;
                state = State::Text;
            }
            _ => {
                state = State::Text;
            }
        }
    });

    println!("Answer 2: {result}");
}