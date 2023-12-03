use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let total = input.lines().map(|line| {
        let mut line_it = (0..line.len()).filter_map(|index| {
            let substr = &line[index..];
            let result = if substr.starts_with("one") {
                Some(1)
            } else if substr.starts_with("two") {
                Some(2)
            } else if substr.starts_with("three") {
                Some(3)
            } else if substr.starts_with("four") {
                Some(4)
            } else if substr.starts_with("five") {
                Some(5)
            } else if substr.starts_with("six") {
                Some(6)
            } else if substr.starts_with("seven") {
                Some(7)
            } else if substr.starts_with("eight") {
                Some(8)
            } else if substr.starts_with("nine") {
                Some(9)
            } else {
                substr.chars().next().unwrap().to_digit(10)
            };
            result
        });

        let first = line_it.next().expect("should be a number");
        match line_it.last() {
            Some(num) => first * 10 + num,
            None => first * 10 + first
        }
    }).sum::<u32>();

    return Ok(total.to_string());
}

#[tracing::instrument]
pub fn process_statemachine_broken(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut total = 0;
    for line in input.lines() {
        //print!("{} => ", line);
        let mut first_digit = '-';
        let mut last_digit = '-';
        let mut state = State::Start;

        for c in line.chars() {
            if c.is_digit(10) && c != '0' {
                //print!("{} ", c);
                state = State::Start;
                if first_digit == '-' { first_digit = c; }
                last_digit = c;
            } else {
                state = match state {
                    State::Start => match c {
                        'e' => State::E,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::E => match c {
                        'i' => State::EI,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::EI => match c {
                        'g' => State::EIG,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::EIG => match c {
                        'h' => State::EIGH,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::EIGH => match c {
                        't' => {
                            //print!("eight ");
                            if first_digit == '-' { first_digit = '8'; }
                            last_digit = '8';
                            State::T
                        }
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        _ => State::Start
                    },
                    State::F => match c {
                        'i' => State::FI,
                        'o' => State::FO,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::FI => match c {
                        'v' => State::FIV,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::FIV => match c {
                        'e' => {
                            if first_digit == '-' { first_digit = '5'; }
                            last_digit = '5';
                            State::E
                        }
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::FO => match c {
                        'u' => State::FOU,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::FOU => match c {
                        'r' => {
                            //print!("four ");
                            if first_digit == '-' { first_digit = '4'; }
                            last_digit = '4';
                            State::Start
                        }
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::N => match c {
                        'i' => State::NI,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::NI => match c {
                        'n' => State::NIN,
                        'e' => State::SE,
                        'f' => State::F,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::NIN => match c {
                        'e' => {
                            //print!("nine ");
                            if first_digit == '-' { first_digit = '9'; }
                            last_digit = '9';
                            State::E
                        }
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::O => match c {
                        'n' => State::ON,
                        'e' => State::SE,
                        'f' => State::F,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::ON => match c {
                        'e' => {
                            //print!("one ");
                            if first_digit == '-' { first_digit = '1'; }
                            last_digit = '1';
                            State::E
                        }
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::S => match c {
                        'i' => State::SI,
                        'e' => State::SE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::SI => match c {
                        'x' => {
                            //print!("six ");
                            if first_digit == '-' { first_digit = '6'; }
                            last_digit = '6';
                            State::Start
                        }
                        'e' => State::E,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::SE => match c {
                        'v' => State::SEV,
                        'i' => State::EI,
                        'e' => State::E,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::SEV => match c {
                        'e' => State::SEVE,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::SEVE => match c {
                        'n' => {
                            //print!("seven ");
                            if first_digit == '-' { first_digit = '7'; }
                            last_digit = '7';
                            State::N
                        }
                        'i' => State::EI,
                        'e' => State::E,
                        'f' => State::F,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::T => match c {
                        'h' => {
                            State::TH
                        }
                        'w' => {
                            State::TW
                        }
                        'e' => State::E,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::TH => match c {
                        'r' => {
                            State::THR
                        }
                        'e' => State::E,
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::THR => match c {
                        'e' => {
                            State::THRE
                        }
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::THRE => match c {
                        'e' => {
                            //print!("three ");
                            if first_digit == '-' { first_digit = '3'; }
                            last_digit = '3';
                            State::E
                        }
                        'f' => State::F,
                        'n' => State::N,
                        'o' => State::O,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    },
                    State::TW => match c {
                        'o' => {
                            //print!("two ");
                            if first_digit == '-' { first_digit = '2'; }
                            last_digit = '2';
                            State::O
                        }
                        'e' => State::E,
                        'f' => State::F,
                        'n' => State::N,
                        's' => State::S,
                        't' => State::T,
                        _ => State::Start
                    }
                };
            }
        }

        //println!("{}{}", &first_digit, &last_digit);
        total += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }

    return Result::Ok(total.to_string());
}


enum State {
    Start,
    // eight
    E,
    EI,
    EIG,
    EIGH,
    // four, five
    F,
    FO,
    FOU,
    FI,
    FIV,
    // nine
    N,
    NI,
    NIN,
    // one,
    O,
    ON,
    // six, seven
    S,
    SI,
    SE,
    SEV,
    SEVE,
    // two, three
    T,
    TW,
    TH,
    THR,
    THRE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}