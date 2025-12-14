use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample, 10)?, 40);
    println!("Part One: {}", part_one(actual, 1000)?);

    assert_eq!(part_two(sample)?, 25272f64);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

fn calculate_distance(b1: &JunctionBox, b2: &JunctionBox) -> f64 {
    let JunctionBox {
        x: x1,
        y: y1,
        z: z1,
        ..
    } = b1;
    let JunctionBox {
        x: x2,
        y: y2,
        z: z2,
        ..
    } = b2;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

#[derive(Debug)]
struct JunctionBox<'a> {
    id: &'a str,
    x: f64,
    y: f64,
    z: f64,
    circuit: Circuit,
}

impl PartialEq for JunctionBox<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for JunctionBox<'_> {}

#[derive(Debug)]
struct Connection {
    /// Index in the array for item a
    a: usize,
    /// Index in the array for item b
    b: usize,
    distance: f64,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Eq for Connection {}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

type Circuit = u32;

fn part_one(file: &str, iterations: usize) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut boxes: Vec<JunctionBox> = lines
        .into_iter()
        .enumerate()
        .map(|(index, l)| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().parse::<f64>().unwrap();
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            let z = parts.next().unwrap().parse::<f64>().unwrap();
            JunctionBox {
                id: l,
                x,
                y,
                z,
                circuit: index as u32,
            }
        })
        .collect();

    // build a list of all the connections
    let mut connections: Vec<Connection> = Vec::new();
    let len = boxes.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let distance = calculate_distance(boxes.get(i).unwrap(), boxes.get(j).unwrap());
            connections.push(Connection {
                a: i,
                b: j,
                distance,
            });
        }
    }
    connections.sort();

    for counter in 0..iterations {
        let shortest = connections.get(counter).unwrap();

        let mut replacement_circuit = None;
        {
            let a = boxes.get(shortest.a).unwrap();
            let b = boxes.get(shortest.b).unwrap();

            if a.circuit != b.circuit {
                replacement_circuit = Some((a.circuit, b.circuit));
            }
        }

        if let Some((from, to)) = replacement_circuit {
            for other in boxes.iter_mut() {
                if other.circuit == from {
                    other.circuit = to;
                }
            }
        }
    }

    let mut circuit_sizes: HashMap<u32, u64> = HashMap::new();
    for b in boxes {
        circuit_sizes
            .entry(b.circuit)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut all_sizes: Vec<u64> = circuit_sizes.into_values().collect();
    all_sizes.sort();
    let highest_three = all_sizes.iter().rev().take(3).product();

    Ok(highest_three)
}

fn part_two(file: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let mut boxes: Vec<JunctionBox> = lines
        .into_iter()
        .enumerate()
        .map(|(index, l)| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().parse::<f64>().unwrap();
            let y = parts.next().unwrap().parse::<f64>().unwrap();
            let z = parts.next().unwrap().parse::<f64>().unwrap();
            JunctionBox {
                id: l,
                x,
                y,
                z,
                circuit: index as u32,
            }
        })
        .collect();

    // build a list of all the connections
    let mut connections: Vec<Connection> = Vec::new();
    let len = boxes.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let distance = calculate_distance(boxes.get(i).unwrap(), boxes.get(j).unwrap());
            connections.push(Connection {
                a: i,
                b: j,
                distance,
            });
        }
    }
    connections.sort();

    for connection in connections {
        let mut replacement_circuit = None;
        {
            let a = boxes.get(connection.a).unwrap();
            let b = boxes.get(connection.b).unwrap();

            if a.circuit != b.circuit {
                replacement_circuit = Some((a.circuit, b.circuit));
            }
        }

        if let Some((from, to)) = replacement_circuit {
            for other in boxes.iter_mut() {
                if other.circuit == from {
                    other.circuit = to;
                }
            }
        }

        let first_circuit = boxes.first().unwrap().circuit;
        if boxes.iter().all(|b| b.circuit == first_circuit) {
            let a = boxes.get(connection.a).unwrap();
            let b = boxes.get(connection.b).unwrap();
            return Ok(a.x * b.x);
        }
    }

    Err("Ran out of connections".into())
}
