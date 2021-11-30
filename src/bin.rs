use itertools::Itertools;
use structopt::StructOpt;

use utils::Error;
use utils::Puzzle;

mod solutions;
use solutions::*;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short = "d", long = "days")]
    days: Option<Vec<i32>>,
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();

    let solutions: Vec<Box<dyn Puzzle>> = vec![
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
        Box::new(Day06),
        Box::new(Day07),
        Box::new(Day08),
        Box::new(Day09),
        Box::new(Day10),
        Box::new(Day11),
        Box::new(Day12),
        Box::new(Day13),
        Box::new(Day14),
        Box::new(Day15),
        Box::new(Day16),
        Box::new(Day17),
        Box::new(Day18),
        Box::new(Day19),
        Box::new(Day20),
        Box::new(Day21),
        Box::new(Day22),
        Box::new(Day23),
        Box::new(Day24),
        Box::new(Day25),
    ];

    let days: Vec<i32> = opt.days.unwrap_or_else(|| {
        solutions
            .iter()
            .filter(|&p| p.implemented())
            .last()
            .iter()
            .map(|&p| p.day())
            .collect::<Vec<i32>>()
    });

    days.iter()
        .cloned()
        .cartesian_product(vec![1, 2])
        .map(|(day, part)| -> (i32,i32,Result<Box<dyn std::fmt::Debug>, Error>) {
            let res = solutions
                .iter()
                .filter(|&p| p.day() == day)
                .next()
                .ok_or(Error::NoneError)
                .and_then(|puzzle| puzzle.call_part(part));
            (day, part, res)
        })
        .inspect(|(day, part, res)| {
            println!("Day {:02}, Part {:}", day, part);
            println!("{:?}", res);
        } ).map(|(_day,_part,res)| res)
        .collect::<Result<Vec<_>,Error>>()?;

    Ok(())
}
