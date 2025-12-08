fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 4277556);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 3263827);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

struct Problem {
    numbers: Vec<u64>,
    solution: Option<u64>,
}

impl Problem {
    pub fn new() -> Self {
        Self {
            numbers: vec![],
            solution: None,
        }
    }

    pub fn push(&mut self, n: u64) {
        self.numbers.push(n);
    }

    pub fn add(&mut self) {
        self.solution = Some(self.numbers.iter().sum::<u64>());
    }

    pub fn multiply(&mut self) {
        self.solution = Some(self.numbers.iter().product::<u64>());
    }
}

fn part_one(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();

    let problems = lines.fold(Vec::<Problem>::new(), |mut acc, line| {
        let mut problem_index = 0;
        for section in line.split(' ') {
            if section.is_empty() {
                continue;
            };
            let problem = {
                if acc.len() <= problem_index {
                    acc.push(Problem::new());
                };
                acc.get_mut(problem_index).unwrap()
            };
            match section {
                "+" => problem.add(),
                "*" => problem.multiply(),
                n => {
                    let n = n.parse::<u64>().unwrap();
                    problem.push(n);
                }
            };
            problem_index += 1;
        }

        acc
    });

    let total = problems
        .into_iter()
        .fold(0, |acc, n| acc + n.solution.unwrap());

    Ok(total)
}

struct ColumnProblem {
    numbers: Vec<u64>,
    operator: Option<char>,
    total: u64,
}

impl ColumnProblem {
    pub fn new() -> Self {
        Self {
            numbers: vec![],
            operator: None,
            total: 0,
        }
    }

    pub fn add_number(&mut self, n: u64) {
        self.numbers.push(n);
    }

    pub fn set_operator(&mut self, c: char) {
        if self.operator.is_some() {
            panic!("Operator already set");
        }
        self.operator = Some(c);
    }

    pub fn calculate(&mut self) -> u64 {
        match self.operator {
            Some('+') => self.total += self.numbers.iter().sum::<u64>(),
            Some('*') => self.total += self.numbers.iter().product::<u64>(),
            _ => panic!("No operator to calculate"),
        }
        self.operator = None;
        self.numbers.clear();
        self.total
    }
}

fn part_two(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut columns: Vec<String> = lines.fold(Vec::new(), |mut acc, line| {
        for (index, char) in line.chars().enumerate() {
            match acc.get_mut(index) {
                Some(s) => s.push(char),
                _ => acc.push(char.to_string()),
            };
        }
        acc
    });
    columns.reverse();

    let mut problem = ColumnProblem::new();
    for column in columns {
        let mut column = column.trim();
        if column.is_empty() {
            problem.calculate();
            continue;
        }

        if let Some(c) = column.strip_suffix('+') {
            problem.set_operator('+');
            column = c.trim();
        }
        if let Some(c) = column.strip_suffix('*') {
            problem.set_operator('*');
            column = c.trim();
        }
        let num = column.parse::<u64>().unwrap();
        problem.add_number(num);
    }

    Ok(problem.calculate())
}
