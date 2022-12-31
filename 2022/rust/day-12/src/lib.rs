pub fn process_part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();
    print!("{} x {}\n", width, height);

    let mut map: Vec<u8> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'a'..='z' => {
                    map.push(c as u8 - 'a' as u8);
                }
                'S' => {
                    map.push(0);
                    start = (x, y);
                }
                'E' => {
                    map.push('z' as u8 - 'a' as u8);
                    goal = (x, y);
                }
                _ => {
                    panic!("unexpected input");
                }
            }
        }
    }

    for y in 0..height {
        print!("{:>2}: ", y);
        for x in 0..width {
            if start == (x, y) {
                print!(">{:>2}<", map[y * width + x]);
            } else if goal == (x, y) {
                print!("<{:>2}>", map[y * width + x]);
            } else {
                print!(" {:>2} ", map[y * width + x]);
            }
        }
        print!("\n");
    }

    let mut open = vec![start];
    let mut closed: Vec<(usize, usize)> = Vec::new();

    while let current = open.pop {}

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
