use crate::day3::INPUT;

#[derive(Debug)]
enum State {
    Text,
    Operator(usize),
    OpenParen,
    FirstNumber(u32),
    Comma,
    SecondNumber(u32),
}

pub fn part1() {
    let input = INPUT;

    let mut result = 0;

    let mut state = State::Text;

    let mut first_number = 0u32;

    input.chars().for_each(|c| {
        match state {
            State::Text if c == 'm' => {
                state = State::Operator(0);
            }
            State::Operator(index) if index == 0 && c == 'u' => {
                state = State::Operator(1);
            }
            State::Operator(index) if index == 1 && c == 'l' => {
                state = State::Operator(2);
            }
            State::Operator(index) if index == 2 && c == '(' => {
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

    println!("Answer 1: {result}");
}