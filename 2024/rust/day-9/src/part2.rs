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
                disk_map.push(i as i64 / 2);
            }
        } else {
            for _ in 0..(c.to_digit(10).unwrap()) {
                disk_map.push(-1);
            }
        }
    }

    //dump_disk_map(&disk_map);

    let mut last_idx = disk_map.len() - 1;
    let mut file_to_move = disk_map[last_idx] + 1;
    while file_to_move > 0 {
        file_to_move -= 1;

        // find file from right
        let mut i = last_idx;
        while i > 0 && disk_map[i] != file_to_move {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        let file_end = i;
        while i > 0 && disk_map[i - 1] == file_to_move {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        last_idx = i;
        let file_start = i;
        let file_size = file_end - file_start + 1;

        i = 0;
        while let Some((start, len)) = find_next_space(&disk_map, i) {
            i = start + len;
            if start > file_start {
                break;
            }
            if len < file_size {
                continue;
            }

            // move all file blocks
            for off in 0..file_size {
                disk_map[start + off] = disk_map[file_start + off];
                disk_map[file_start + off] = -1;
            }
            break;
        }
    }
    //dump_disk_map(&disk_map);

    let mut checksum = 0;
    for (i, e) in disk_map.into_iter().enumerate() {
        if e == -1 {
            continue;
        }
        checksum += i as i128 * e as i128;
    }
    Ok(checksum.to_string())
}

//fn dump_disk_map(disk_map: &Vec<i64>) {
//    for e in disk_map {
//        if *e == -1 {
//            print!(". ");
//        } else {
//            print!("{} ", *e);
//        }
//    }
//    println!("");
//}

fn find_next_space(disk_map: &Vec<i64>, start: usize) -> Option<(usize, usize)> {
    let mut i = start;
    while i < disk_map.len() && disk_map[i] != -1 {
        i += 1;
    }
    if i == disk_map.len() {
        return None;
    }
    let space_start = i;
    while i < disk_map.len() && disk_map[i] == -1 {
        i += 1;
    }
    return Some((space_start, i - space_start));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
