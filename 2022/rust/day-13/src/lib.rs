#![feature(is_some_and)]

pub fn process_part1(input: &str) -> String {
    let mut sum_ordered_pair_numbers = 0;

    for (pair_index, pair) in input.split("\n\n").enumerate() {
        if let Some((l, r)) = pair.split_once("\n") {
            let pkg1 = tokenize(l);
            let pkg2 = tokenize(r);

            let pair_number = pair_index + 1;
            if packet_in_right_order(pkg1, pkg2) {
                print!(
                    "Pair {} -- OK, sum: {} -> {}\n",
                    pair_number,
                    sum_ordered_pair_numbers,
                    sum_ordered_pair_numbers + pair_number
                );

                sum_ordered_pair_numbers += pair_number;
            } else {
                print!("Pair {} !!\n", pair_number);
            }
        }
    }

    return sum_ordered_pair_numbers.to_string();
}

fn packet_in_right_order(mut left: Vec<PacketToken>, mut right: Vec<PacketToken>) -> bool {
    let mut left_index = 0;
    let mut right_index = 0;

    loop {
        if let Some(left_item) = left.get(left_index) {
            if let Some(right_item) = right.get(right_index) {
                print!(" > {:?} vs. {:?}\n", &left_item, &right_item);
                match left_item {
                    PacketToken::Number(left_n) => match right_item {
                        PacketToken::Number(right_n) => {
                            if left_n < right_n {
                                return true;
                            }
                            if left_n > right_n {
                                return false;
                            }
                            left_index += 1;
                            right_index += 1;
                        }
                        PacketToken::ListStart => {
                            left.insert(left_index + 1, PacketToken::ListEnd);
                            left.insert(left_index, PacketToken::ListStart);
                        }
                        PacketToken::ListEnd => return false,
                    },
                    PacketToken::ListStart => match right_item {
                        PacketToken::ListStart => {
                            left_index += 1;
                            right_index += 1;
                        }
                        PacketToken::ListEnd => return false,
                        PacketToken::Number(_) => {
                            right.insert(right_index + 1, PacketToken::ListEnd);
                            right.insert(right_index, PacketToken::ListStart);
                        }
                    },
                    PacketToken::ListEnd => match right_item {
                        PacketToken::Number(_) => return true,
                        PacketToken::ListStart => return true,
                        PacketToken::ListEnd => {
                            left_index += 1;
                            right_index += 1;
                        }
                    },
                }
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}

fn tokenize(packet: &str) -> Vec<PacketToken> {
    let mut result = Vec::new();

    let mut collect = u32::MAX;

    for c in packet.chars() {
        // handle (multi digit) numbers first and skip to next iteration
        if c.is_digit(10) {
            let i_digit = c.to_digit(10).unwrap();
            if collect == u32::MAX {
                collect = i_digit;
            } else {
                collect = collect * 10 + i_digit;
            }
            continue;
        }

        // c is not a digit.. check for and handle collected number
        if collect != u32::MAX {
            if result
                .last()
                .is_some_and(|l| !matches!(l, PacketToken::ListStart))
            {
                print!(",");
            }
            result.push(PacketToken::Number(collect));
            print!("{}", collect);
            collect = u32::MAX;
        }

        if c == '[' {
            if result
                .last()
                .is_some_and(|l| !matches!(l, PacketToken::ListStart))
            {
                print!(",");
            }
            result.push(PacketToken::ListStart);
            print!("[");
        } else if c == ']' {
            result.push(PacketToken::ListEnd);
            print!("]");
        }
    }
    print!("\n");

    return result;
}

#[derive(PartialEq, Eq, Debug)]
enum PacketToken {
    Number(u32),
    ListStart,
    ListEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "13");
    }

    const TESTINPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
