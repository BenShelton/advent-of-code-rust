fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 3);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 14);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    pub fn contains(&self, index: &u64) -> bool {
        &self.start <= index && index <= &self.end
    }

    pub fn count(&self) -> u64 {
        self.end - self.start + 1
    }
}

fn part_one(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut total: u64 = 0;

    let mut ranges: Vec<Range> = vec![];
    let mut checking_ids = false;
    for line in lines {
        if line.is_empty() {
            checking_ids = true;
            continue;
        }

        if !checking_ids {
            if let Some((Ok(start), Ok(end))) = line
                .split_once('-')
                .map(|(s, e)| (s.parse::<u64>(), e.parse::<u64>()))
            {
                ranges.push(Range { start, end });
            }
        } else if let Ok(index) = line.parse::<u64>()
            && ranges.iter().any(|r| r.contains(&index))
        {
            total += 1;
        }
    }

    Ok(total)
}

fn part_two(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut total: u64 = 0;

    let mut ranges: Vec<Range> = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }

        let (start, end) = line
            .split_once('-')
            .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
            .unwrap();

        ranges.push(Range { start, end });
    }

    ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range> = vec![];
    for r in ranges {
        if let Some(last) = merged.last_mut()
            && r.start <= last.end + 1
        {
            last.end = last.end.max(r.end);
        } else {
            merged.push(r);
        }
    }

    for r in merged {
        total += r.count();
    }

    Ok(total)
}
