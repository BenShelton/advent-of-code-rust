fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 357);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 3_121_910_778_619);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

#[allow(clippy::unnecessary_wraps)]
fn part_one(file: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut total: u32 = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut highest_start = 0;
        let mut highest_end = 0;
        for window in digits.windows(2) {
            let current = window[0];
            let next = window[1];
            if current > highest_start {
                highest_end = next;
                highest_start = current;
            } else {
                if current > highest_end {
                    highest_end = current;
                }
                if next > highest_end {
                    highest_end = next;
                }
            }
        }

        let line_total = (highest_start * 10) + highest_end;
        total += line_total;
    }

    Ok(total)
}

#[allow(clippy::unnecessary_wraps)]
fn part_two(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut total: u64 = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let digit_len = digits.len();
        let mut start_index = 0;
        let mut line_total: u64 = 0;
        for i in 0..12 {
            let mut highest_in_range = 0;
            let digit_range = start_index..(digit_len - (11 - i));
            for j in digit_range {
                let current = digits[j];
                if current > highest_in_range {
                    highest_in_range = current;
                    start_index = j + 1;
                }
            }
            line_total += u64::from(highest_in_range) * 10u64.pow(11 - u32::try_from(i).unwrap());
        }

        total += line_total;
    }

    Ok(total)
}
