use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 21);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 40);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

struct Beam {
    columns: HashSet<usize>,
    next_columns: HashSet<usize>,
    splits: u64,
}

impl Beam {
    pub fn new() -> Self {
        Self {
            columns: HashSet::new(),
            next_columns: HashSet::new(),
            splits: 0,
        }
    }

    pub fn start(&mut self, col: usize) {
        self.next_columns.insert(col);
    }

    pub fn travel(&mut self) {
        self.columns = self.next_columns.clone();
    }

    pub fn split(&mut self, col: usize) {
        if self.columns.contains(&col) {
            self.next_columns.remove(&col);
            self.next_columns.insert(col - 1);
            self.next_columns.insert(col + 1);
            self.splits += 1;
        }
    }

    pub fn get_splits(&self) -> u64 {
        self.splits
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part_one(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();

    let mut beam = Beam::new();
    for line in lines {
        for (col, char) in line.chars().enumerate() {
            match char {
                'S' => beam.start(col),
                '^' => beam.split(col),
                _ => {}
            }
        }
        beam.travel();
    }

    Ok(beam.get_splits())
}

struct TimelineBeam {
    timelines: HashMap<usize, u64>,
    next_timelines: HashMap<usize, u64>,
}

impl TimelineBeam {
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
            next_timelines: HashMap::new(),
        }
    }

    pub fn start(&mut self, col: usize) {
        self.next_timelines.insert(col, 1);
    }

    pub fn split(&mut self, col: usize) {
        if let Some(count) = self.timelines.get(&col)
            && count > &0
        {
            self.next_timelines
                .entry(col - 1)
                .and_modify(|c| *c += count)
                .or_insert(*count);
            self.next_timelines
                .entry(col + 1)
                .and_modify(|c| *c += count)
                .or_insert(*count);
            self.next_timelines.insert(col, 0);
        }
    }

    pub fn travel(&mut self) {
        self.timelines = self.next_timelines.clone();
    }

    pub fn count_timelines(&self) -> u64 {
        self.timelines.values().sum()
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part_two(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();

    let mut beam = TimelineBeam::new();
    for line in lines {
        for (col, char) in line.chars().enumerate() {
            match char {
                'S' => beam.start(col),
                '^' => beam.split(col),
                _ => {}
            }
        }
        beam.travel();
    }

    Ok(beam.count_timelines())
}
