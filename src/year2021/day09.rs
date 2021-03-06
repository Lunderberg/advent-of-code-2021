#![allow(unused_imports)]
use crate::utils::{Adjacency, GridMap, GridPos};
use crate::{Error, Puzzle};

use itertools::Itertools;

use std::collections::HashSet;

pub struct Day09;

#[derive(Debug)]
pub struct HeightMap {
    map: GridMap<u8>,
}

impl HeightMap {
    fn adjacent_points(
        &self,
        pos: GridPos,
    ) -> impl Iterator<Item = GridPos> + '_ {
        self.map.adjacent_points(pos, Adjacency::Rook)
    }

    fn low_points(&self) -> impl Iterator<Item = (GridPos, u8)> + '_ {
        self.map.iter().map(|(pos, height)| (pos, *height)).filter(
            move |&(pos, height)| {
                self.adjacent_points(pos).all(|adj| self.map[adj] > height)
            },
        )
    }

    fn basin_points(&self, low_point: GridPos) -> Vec<GridPos> {
        let mut search_stack: Vec<GridPos> = vec![low_point];

        let mut touched: HashSet<_> = HashSet::new();
        touched.insert(low_point);

        let mut output: Vec<GridPos> = Vec::new();

        while search_stack.len() > 0 {
            let point = search_stack.pop().unwrap();
            if self.map[point] != 9 {
                output.push(point);
                self.adjacent_points(point)
                    .filter(|adj| !touched.contains(&adj))
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|adj| {
                        search_stack.push(*adj);
                        touched.insert(*adj);
                    });
            }
        }
        output
    }
}

impl std::str::FromStr for HeightMap {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HeightMap {
            map: s.lines().collect(),
        })
    }
}

impl Puzzle for Day09 {
    const YEAR: u32 = 2021;
    const DAY: u8 = 9;
    const IMPLEMENTED: bool = true;
    const EXAMPLE_NUM: u8 = 0;

    type ParsedInput = HeightMap;
    fn parse_input<'a>(
        lines: impl Iterator<Item = &'a str>,
    ) -> Result<Self::ParsedInput, Error> {
        Ok(HeightMap {
            map: lines.collect(),
        })
    }

    type Part1Result = u64;
    fn part_1(
        height_map: &Self::ParsedInput,
    ) -> Result<Self::Part1Result, Error> {
        Ok(height_map
            .low_points()
            .map(|(_pos, height)| (height + 1) as u64)
            .sum::<u64>())
    }

    type Part2Result = usize;
    fn part_2(
        height_map: &Self::ParsedInput,
    ) -> Result<Self::Part2Result, Error> {
        Ok(height_map
            .low_points()
            .map(|(pos, _height)| height_map.basin_points(pos).len())
            .sorted_by_key(|&i| -(i as i64))
            .take(3)
            .product::<usize>())
    }
}
