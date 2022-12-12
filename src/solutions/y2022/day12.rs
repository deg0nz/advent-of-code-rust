use std::{collections::HashSet, fmt::Display};

use array2d::Array2D;
use priority_queue::PriorityQueue;

use crate::solutions::{answer::Answer, Solution};

pub struct Day12;

impl Solution for Day12 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let (map, start, end, _) = parse(input);

        println!("Searching from {start} to {end}");
        println!(
            "Cost of z: {}. End: {}",
            map.get(2, 4).unwrap(),
            map.get(2, 5).unwrap()
        );

        search_distance(start, end, &map).map(|x| x.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let (map, _, end, locs) = parse(input);

        Some(
            locs.iter()
                .map(|start| search_distance(*start, end, &map))
                .filter_map(|x| x)
                .min()
                .unwrap()
                .into(),
        )
    }
}

fn search_distance(start: Location, end: Location, map: &Array2D<u8>) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue: PriorityQueue<Location, isize> = PriorityQueue::new();
    queue.push(start, 0);
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((current, cost)) = queue.pop() {
        if current == end {
            return Some(cost.abs() as usize);
        }

        for (col, row) in directions.iter() {
            let x = current.col as isize + col;
            let y = current.row as isize + row;

            if is_in_map(x, y, &map) {
                let neighbor = Location::new(x as usize, y as usize);
                if !visited.contains(&neighbor)
                    && *map.get(neighbor.row, neighbor.col).unwrap()
                        <= *map.get(current.row, current.col).unwrap() + 1
                {
                    visited.insert(neighbor);
                    queue.push(neighbor, cost - 1);
                }
            }
        }
    }
    None
}

fn is_in_map(col: isize, row: isize, map: &Array2D<u8>) -> bool {
    0 <= col && col < map.row_len() as isize && 0 <= row && row < map.column_len() as isize
}

fn parse(input: &str) -> (Array2D<u8>, Location, Location, Vec<Location>) {
    let input = input.trim_end();
    let mut map = Array2D::filled_with(
        0,
        input.lines().count(),
        input.lines().next().unwrap().len(),
    );

    let (mut start, mut end) = (Location::default(), Location::default());
    let mut lowest_locations = Vec::new();

    input.lines().enumerate().for_each(|(row, l)| {
        l.char_indices().for_each(|(col, c)| {
            if c == 'S' {
                start = Location::new(col, row);
                lowest_locations.push(start);
            } else if c == 'E' {
                end = Location::new(col, row);
                map.set(row, col, 25).unwrap();
            } else {
                map.set(row, col, (c as u8) - b'a').unwrap();
                if c == 'a' {
                    lowest_locations.push(Location::new(col, row));
                }
            }
        })
    });

    (map, start, end, lowest_locations)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Location {
    col: usize,
    row: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.col, self.row)
    }
}

impl Location {
    fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

#[cfg(test)]
mod test {
    use crate::{utils::load_sample, Year};

    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            Day12.solve_a(load_sample(Year::Y2022, "12.txt").unwrap().as_str()),
            Some(Answer::UInt(31))
        )
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Day12.solve_b(load_sample(Year::Y2022, "12.txt").unwrap().as_str()),
            Some(Answer::UInt(29))
        )
    }
}
