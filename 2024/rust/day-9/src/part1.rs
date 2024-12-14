use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut disk_map = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if !c.is_digit(10) {
            break;
        }
        if i % 2 == 0 {
            for _ in 0..(c.to_digit(10).unwrap()) {
                disk_map.push(i as i32 / 2);
            }
        } else {
            for _ in 0..(c.to_digit(10).unwrap()) {
                disk_map.push(-1);
            }
        }
    }

    //for e in &disk_map {
    //    if *e == -1 {
    //        print!(".");
    //    } else {
    //        print!("{}", *e);
    //    }
    //}
    //println!("");

    let (mut left, mut right) = (0, disk_map.len() - 1);

    while left < right {
        while disk_map[left] != -1 {
            left += 1;
        }

        while disk_map[right] == -1 {
            right -= 1;
        }
        disk_map[left] = disk_map[right];
        disk_map.remove(right);
        right -= 1;
    }

    //for e in &disk_map {
    //    if *e == -1 {
    //        print!(".");
    //    } else {
    //        print!("{}", *e);
    //    }
    //}
    //println!("");

    let mut checksum = 0;
    for (i, e) in disk_map.into_iter().enumerate() {
        if e == -1 {
            break;
        }
        checksum += i as i64 * e as i64;
    }
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
