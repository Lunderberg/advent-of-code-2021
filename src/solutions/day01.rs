use utils::Error;
use utils::Puzzle;

pub struct Day01;

impl Puzzle for Day01 {
    fn day(&self) -> i32 {
        1
    }
    fn implemented(&self) -> bool {
        true
    }
    fn part_1(&self) -> Result<Box<dyn std::fmt::Debug>, Error> {
        let result = ();
        Ok(Box::new(result))
    }
    fn part_2(&self) -> Result<Box<dyn std::fmt::Debug>, Error> {
        let result = ();
        Ok(Box::new(result))
    }
}
