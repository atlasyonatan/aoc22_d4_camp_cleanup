use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;


fn main() {
    let path = Path::new("../input.txt");

    //part 1
    {
        let file = File::open(path).unwrap();
        let fully_contained = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .map(|line| {
                let mut i = line.split(',').map(|s| {
                    let mut i = s.split('-').map(|s| s.parse::<u32>().unwrap());
                    i.next().unwrap()..=i.next().unwrap()
                });
                (i.next().unwrap(), i.next().unwrap())
            })
            .filter(|(r1, r2)| r1.fully_contains(r2) || r2.fully_contains(r1))
            .count();
        println!("part 1: {}", fully_contained);
    }

    //part 1
    {
        let file = File::open(path).unwrap();
        let overlapping = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .map(|line| {
                let mut i = line.split(',').map(|s| {
                    let mut i = s.split('-').map(|s| s.parse::<u32>().unwrap());
                    i.next().unwrap()..=i.next().unwrap()
                });
                (i.next().unwrap(), i.next().unwrap())
            })
            .filter(|(r1, r2)| r1.overlaps(r2))
            .count();
        println!("part 2: {}", overlapping);
    }
}

trait FullyContains<T> {
    fn fully_contains(&self, other: &T) -> bool;
}

impl<T: PartialOrd> FullyContains<RangeInclusive<T>> for RangeInclusive<T> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.contains(&other.start()) && self.contains(&other.end())
    }
}
trait Overlaps<T> {
    fn overlaps(&self, other: &T) -> bool;
}

impl<T: PartialOrd> Overlaps<RangeInclusive<T>> for RangeInclusive<T> {
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(&other.start())
            || self.contains(&other.end())
            || other.contains(&self.start())
            || other.contains(&self.end())
    }
}
