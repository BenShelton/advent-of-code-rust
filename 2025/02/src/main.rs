fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_sample = include_str!("sample-input.txt");
    let file = include_str!("input.txt");

    assert_eq!(part_one(file_sample)?, 1227775554);
    println!("Part One: {}", part_one(file)?);

    assert_eq!(part_two(file_sample)?, 4174379265);
    println!("Part Two: {}", part_two(file)?);

    Ok(())
}

fn part_one(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let line = file.trim();
    let ids = line.split(',');
    let mut total = 0;
    for id in ids {
        let (first_id, last_id) = id.split_once('-').unwrap();
        for i in first_id.parse::<u64>()?..=last_id.parse::<u64>()? {
            let s = i.to_string();
            let len = s.len();
            if len % 2 != 0 {
                continue;
            }
            let (first_half, second_half) = s.split_at(len / 2);
            if first_half == second_half {
                total += i;
            }
        }
    }

    Ok(total)
}

fn part_two(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let line = file.trim();
    let ids = line.split(',');
    let mut total = 0;
    for id in ids {
        let (first_id, last_id) = id.split_once('-').unwrap();
        for i in first_id.parse::<u64>()?..=last_id.parse::<u64>()? {
            let s = i.to_string();
            let len = s.len();
            for n in 1..=(len / 2) {
                let current_str = &s[0..n];
                let repeated = current_str.repeat(len / n);
                if repeated == s {
                    total += i;
                    break;
                }
            }
        }
    }

    Ok(total)
}
