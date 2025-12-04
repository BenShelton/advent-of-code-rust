fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 3);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 6);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

#[derive(Debug, PartialEq)]
struct DialResult {
    new_position: i32,
    /// Counts the number of times zero was _passed_ during the rotation.
    zero_passes: u32,
}

fn rotate_dial(current: i32, line: &str) -> Result<DialResult, Box<dyn std::error::Error>> {
    let (dir, num) = line.split_at(1);
    let num = num.parse::<i32>()?;
    let mut zero_passes: u32 = (num / 100) as u32;
    let num = num % 100;
    match dir {
        "R" => {
            let new_position = (current + num) % 100;
            if current != 0 && new_position != 0 && new_position < current {
                zero_passes += 1;
            }
            Ok(DialResult {
                new_position,
                zero_passes,
            })
        }
        "L" => {
            let new_position = {
                if current - num < 0 {
                    100 + (current - num)
                } else {
                    current - num
                }
            };
            if current != 0 && new_position != 0 && new_position > current {
                zero_passes += 1;
            }
            Ok(DialResult {
                new_position,
                zero_passes,
            })
        }
        _ => Ok(DialResult {
            new_position: current,
            zero_passes,
        }),
    }
}

fn part_one(file: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut current_dial: i32 = 50;
    let mut zero_count = 0;

    for line in lines {
        let result = rotate_dial(current_dial, line)?;
        current_dial = result.new_position;
        if current_dial == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count)
}

fn part_two(file: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut current_dial: i32 = 50;
    let mut zero_count: u32 = 0;

    for line in lines {
        let result = rotate_dial(current_dial, line)?;
        current_dial = result.new_position;
        zero_count += result.zero_passes;
        if result.new_position == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_dial() {
        assert_eq!(
            rotate_dial(50, "L68").unwrap(),
            DialResult {
                new_position: 82,
                zero_passes: 1
            }
        );
        assert_eq!(
            rotate_dial(82, "L30").unwrap(),
            DialResult {
                new_position: 52,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(52, "R48").unwrap(),
            DialResult {
                new_position: 0,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(0, "L5").unwrap(),
            DialResult {
                new_position: 95,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(95, "R60").unwrap(),
            DialResult {
                new_position: 55,
                zero_passes: 1
            }
        );
        assert_eq!(
            rotate_dial(55, "L55").unwrap(),
            DialResult {
                new_position: 0,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(0, "L1").unwrap(),
            DialResult {
                new_position: 99,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(99, "L99").unwrap(),
            DialResult {
                new_position: 0,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(0, "R14").unwrap(),
            DialResult {
                new_position: 14,
                zero_passes: 0
            }
        );
        assert_eq!(
            rotate_dial(14, "L82").unwrap(),
            DialResult {
                new_position: 32,
                zero_passes: 1
            }
        );
    }
}
