fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample = include_str!("data-sample.txt").trim();
    let actual = include_str!("data-actual.txt").trim();

    assert_eq!(part_one(sample)?, 50);
    println!("Part One: {}", part_one(actual)?);

    assert_eq!(part_two(sample)?, 24);
    println!("Part Two: {}", part_two(actual)?);

    Ok(())
}

#[derive(Clone)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn area(&self, other: &Tile) -> u64 {
        let x = self.x.abs_diff(other.x) + 1;
        let y = self.y.abs_diff(other.y) + 1;
        x * y
    }

    fn in_polygon(&self, tiles: &[Tile]) -> bool {
        let mut inside = false;
        let len = tiles.len();

        for i in 0..len {
            let a = tiles.get(i).unwrap();
            let b = tiles.get((i + 1) % len).unwrap();

            if tile_on_segment(self, a, b) {
                return true;
            }

            let y1 = a.y;
            let y2 = b.y;

            if (y1 <= self.y && y2 > self.y) || (y2 <= self.y && y1 > self.y) {
                let x_intersect = a.x + (self.y - y1) * (b.x - a.x) / (y2 - y1);

                if x_intersect > self.x {
                    inside = !inside;
                }
            }
        }

        inside
    }
}

struct Rect {
    min: Tile,
    max: Tile,
}

impl Rect {
    fn from_points(tile1: &Tile, tile2: &Tile) -> Self {
        let min_x = tile1.x.min(tile2.x);
        let min_y = tile1.y.min(tile2.y);
        let max_x = tile1.x.max(tile2.x);
        let max_y = tile1.y.max(tile2.y);

        Rect {
            min: Tile { x: min_x, y: min_y },
            max: Tile { x: max_x, y: max_y },
        }
    }

    fn in_polygon(&self, tiles: &[Tile]) -> bool {
        let corners = [
            &self.min,
            &self.max,
            &Tile {
                x: self.min.x,
                y: self.max.y,
            },
            &Tile {
                x: self.max.x,
                y: self.min.y,
            },
        ];

        if !corners.iter().all(|tile| tile.in_polygon(tiles)) {
            return false;
        }

        let rect_edges = [
            (
                &self.min,
                &Tile {
                    x: self.max.x,
                    y: self.min.y,
                },
            ),
            (
                &Tile {
                    x: self.max.x,
                    y: self.min.y,
                },
                &self.max,
            ),
            (
                &self.max,
                &Tile {
                    x: self.min.x,
                    y: self.max.y,
                },
            ),
            (
                &Tile {
                    x: self.min.x,
                    y: self.max.y,
                },
                &self.min,
            ),
        ];

        let len = tiles.len();
        for i in 0..len {
            let a = tiles.get(i).unwrap();
            let b = tiles.get((i + 1) % len).unwrap();

            for (r1, r2) in rect_edges {
                if segments_intersect(r1, r2, a, b) {
                    return false;
                }
            }
        }

        true
    }
}

fn tile_on_segment(p: &Tile, a: &Tile, b: &Tile) -> bool {
    let cross = (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x);
    if cross != 0 {
        return false;
    }

    p.x >= a.x.min(b.x) && p.x <= a.x.max(b.x) && p.y >= a.y.min(b.y) && p.y <= a.y.max(b.y)
}

fn orient(a: &Tile, b: &Tile, c: &Tile) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn segments_intersect(a1: &Tile, a2: &Tile, b1: &Tile, b2: &Tile) -> bool {
    let o1 = orient(a1, a2, b1);
    let o2 = orient(a1, a2, b2);
    let o3 = orient(b1, b2, a1);
    let o4 = orient(b1, b2, a2);

    (o1 > 0 && o2 < 0 || o1 < 0 && o2 > 0) && (o3 > 0 && o4 < 0 || o3 < 0 && o4 > 0)
}

#[allow(clippy::unnecessary_wraps)]
fn part_one(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let tiles: Vec<Tile> = lines
        .into_iter()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Tile {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let mut largest = 0;
    let len = tiles.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let tile_a = tiles.get(i).unwrap();
            let tile_b = tiles.get(j).unwrap();
            let area = tile_a.area(tile_b);
            if area > largest {
                largest = area;
            }
        }
    }

    Ok(largest)
}

#[allow(clippy::unnecessary_wraps)]
fn part_two(file: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let lines = file.lines();
    let tiles: Vec<Tile> = lines
        .into_iter()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Tile {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    let mut largest = 0;
    let len = tiles.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let tile_a = tiles.get(i).unwrap();
            let tile_b = tiles.get(j).unwrap();

            let rect = Rect::from_points(tile_a, tile_b);
            if rect.in_polygon(&tiles) {
                let area = tile_a.area(tile_b);
                if area > largest {
                    largest = area;
                }
            }
        }
    }

    Ok(largest)
}
