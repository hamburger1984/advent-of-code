pub fn process_part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();
    print!("{} x {}\n", width, height);

    for (y, line) in lines.iter().enumerate() {
        print!("{:>2}: ", y);
        for (_x, c) in line.chars().enumerate() {
            match c {
                'a'..='z' => {
                    print!(" {:>2} ", c as u8 - 'a' as u8);
                }
                'S' => {
                    print!(">{:>2}<", 0);
                }
                'E' => {
                    print!("<{:>2}>", 'z' as u8 - 'a' as u8);
                }
                _ => {
                    panic!("unexpected input");
                }
            }
        }
        print!("\n");
    }

    return "nope".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "31");
    }

    const TESTINPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
