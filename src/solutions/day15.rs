#![allow(unused_imports)]
use crate::utils::Error;
use crate::utils::{Adjacency, GridMap, GridPos};
use crate::utils::{Puzzle, PuzzleExtensions, PuzzleInput};

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub struct Day15;

#[derive(Debug)]
struct RiskMap {
    grid: GridMap<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchPointInfo {
    src_to_pos: i64,
    heuristic_to_dest: i64,
    previous_point: Option<GridPos>,
    finalized: bool,
}

impl RiskMap {
    fn enlarge_by(&self, factor: usize) -> Self {
        let grid = self
            .grid
            .iter()
            .map(|(pos, val)| {
                let (x, y) = pos.as_xy(&self.grid);
                (x as usize, y as usize, *val)
            })
            .flat_map(|(x, y, val)| {
                (0..factor).map(move |tile_x| {
                    let x = tile_x * self.grid.x_size + x;
                    let val = (val + (tile_x as u8) - 1) % 9 + 1;
                    (x, y, val)
                })
            })
            .flat_map(|(x, y, val)| {
                (0..factor).map(move |tile_y| {
                    let y = tile_y * self.grid.y_size + y;
                    let val = (val + (tile_y as u8) - 1) % 9 + 1;
                    (x, y, val)
                })
            })
            .collect();
        Self { grid }
    }

    fn path_cost(&self) -> Result<usize, Error> {
        Ok(self
            .find_path(self.grid.top_left(), self.grid.bottom_right())?
            .into_iter()
            .skip(1)
            .map(|pos| self.grid[pos] as usize)
            .sum::<usize>())
    }

    fn adjacent_points(
        &self,
        pos: GridPos,
    ) -> impl Iterator<Item = GridPos> + '_ {
        self.grid.adjacent_points(pos, Adjacency::Rook)
    }

    fn find_path(
        &self,
        source: GridPos,
        dest: GridPos,
    ) -> Result<Vec<GridPos>, Error> {
        let get_heuristic_to_dest =
            |pos: &GridPos| -> i64 { 1 * self.grid.manhattan_dist(pos, &dest) };
        let get_priority = |info: &SearchPointInfo| -> Reverse<i64> {
            Reverse(info.src_to_pos + info.heuristic_to_dest)
        };

        let start_info = SearchPointInfo {
            src_to_pos: 0,
            heuristic_to_dest: get_heuristic_to_dest(&source),
            previous_point: None,
            finalized: false,
        };

        let mut search_queue: PriorityQueue<GridPos, Reverse<i64>> =
            PriorityQueue::new();
        search_queue.push(source, get_priority(&start_info));

        let mut pos_info_map: HashMap<GridPos, SearchPointInfo> =
            HashMap::new();
        pos_info_map.insert(source, start_info);

        while search_queue.len() > 0 {
            let current_pos = search_queue.pop().unwrap().0;

            let current_info = pos_info_map.get_mut(&current_pos).unwrap();
            current_info.finalized = true;

            if current_pos == dest {
                break;
            }

            let src_to_current_pos = current_info.src_to_pos;

            self.adjacent_points(current_pos)
                .map(|pos| -> (GridPos, Option<&SearchPointInfo>) {
                    (pos, pos_info_map.get(&pos))
                })
                .filter(|(_pos, opt_info)| {
                    opt_info.map_or(true, |info| !info.finalized)
                })
                .filter_map(|(pos, opt_info)| {
                    let src_to_pos =
                        src_to_current_pos + (self.grid[pos] as i64);
                    opt_info
                        .map_or(true, |info| src_to_pos < info.src_to_pos)
                        .then(|| (pos, opt_info, src_to_pos))
                })
                .map(|(pos, opt_info, src_to_pos)| {
                    let info: SearchPointInfo = opt_info.map_or_else(
                        || SearchPointInfo {
                            src_to_pos,
                            previous_point: Some(current_pos),
                            heuristic_to_dest: get_heuristic_to_dest(&pos),
                            finalized: false,
                        },
                        |info| SearchPointInfo {
                            src_to_pos,
                            previous_point: Some(current_pos),
                            heuristic_to_dest: info.heuristic_to_dest,
                            finalized: false,
                        },
                    );
                    (pos, info)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(pos, info)| {
                    search_queue.push_increase(pos, get_priority(&info));
                    pos_info_map.insert(pos, info);
                });
        }

        if !pos_info_map.contains_key(&dest) {
            return Err(Error::NoPathToDest);
        }

        Ok(std::iter::successors(Some(dest), |pos| {
            pos_info_map[pos].previous_point
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect())
    }
}

impl Day15 {
    fn parse_inputs(&self) -> Result<RiskMap, Error> {
        //let puzzle_input = self.puzzle_input(PuzzleInput::Example(0))?;
        let puzzle_input = self.puzzle_input(PuzzleInput::User)?;

        let grid = puzzle_input.lines().collect();
        Ok(RiskMap { grid })
    }
}

impl Puzzle for Day15 {
    fn day(&self) -> i32 {
        15
    }
    fn implemented(&self) -> bool {
        true
    }
    fn part_1(&self) -> Result<Box<dyn std::fmt::Debug>, Error> {
        let map = self.parse_inputs()?;
        let result = map.path_cost()?;
        Ok(Box::new(result))
    }
    fn part_2(&self) -> Result<Box<dyn std::fmt::Debug>, Error> {
        let map = self.parse_inputs()?.enlarge_by(5);
        let result = map.path_cost()?;
        Ok(Box::new(result))
    }
}
