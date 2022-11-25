use super::*;

pub fn rect<'a>(
    pending_points: &mut BTreeSet<(i32, i32, Tile<'a>)>,
    x: i32,
    y: i32,
    tile: Tile<'a>,
) -> (i32, i32, i32, i32) {
    let mut dirs: Vec<_> = Direction::all().collect();
    let mut x1 = x;
    let mut y1 = y;
    let mut x2 = x;
    let mut y2 = y;

    while let Some(dir) = dirs.get(0).cloned() {
        let points: Vec<_> = match dir {
            Direction::Up | Direction::Down => (x1..=x2)
                .map(|x| {
                    if dir == Direction::Up {
                        (x, y1 - 1)
                    } else {
                        (x, y2 + 1)
                    }
                })
                .collect(),

            Direction::Left | Direction::Right => (y1..=y2)
                .map(|y| {
                    if dir == Direction::Left {
                        (x1 - 1, y)
                    } else {
                        (x2 + 1, y)
                    }
                })
                .collect(),
        };

        let can_grow = points
            .iter()
            .all(|&(x, y)| pending_points.contains(&(x, y, tile)));

        if can_grow {
            match dir {
                Direction::Up => y1 -= 1,
                Direction::Down => y2 += 1,
                Direction::Left => x1 -= 1,
                Direction::Right => x2 += 1,
            }

            for (x, y) in points {
                assert!(pending_points.remove(&(x, y, tile)));
            }
        } else {
            dirs.remove(0);
        }
    }

    (x1, y1, x2, y2)
}

pub fn line<'a>(
    pending_points: &mut BTreeSet<(i32, i32, Tile<'a>)>,
    x: i32,
    y: i32,
    tile: Tile<'a>,
    matches: impl Fn(i32, i32) -> bool,
) -> (i32, i32, i32, i32) {
    let mut x1 = x;
    let mut y1 = y;
    let mut x2 = x;
    let mut y2 = y;

    // -----

    let mut grow_dir = None;

    for dir in Direction::all() {
        let (x, y) = match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if pending_points.contains(&(x, y, tile)) && matches(x, y) {
            grow_dir = Some(dir);
            break;
        }
    }

    // -----

    while let Some(dir) = grow_dir {
        match dir {
            Direction::Up => y1 -= 1,
            Direction::Down => y2 += 1,
            Direction::Left => x1 -= 1,
            Direction::Right => x2 += 1,
        }

        let (x, y) = match dir {
            Direction::Up => (x, y1 - 1),
            Direction::Down => (y, y2 + 1),
            Direction::Left => (x1 - 1, y),
            Direction::Right => (x2 + 1, y),
        };

        if !pending_points.contains(&(x, y, tile)) || !matches(x, y) {
            break;
        }
    }

    // -----

    for px in x1..=x2 {
        for py in y1..=y2 {
            if px == x && py == y {
                continue;
            }

            assert!(pending_points.remove(&(px, py, tile)));
        }
    }

    (x1, y1, x2, y2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right].into_iter()
    }
}
