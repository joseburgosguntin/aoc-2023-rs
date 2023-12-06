use nom::character::complete::{alpha1, newline, space1, u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::{bytes::complete::tag, IResult};

fn main() {
    let text = include_str!("./day_5.txt").trim();
    println!("{}", solution(text));
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tuple((tag("seeds:"), space1)),
        separated_list1(space1, u64),
        tuple((newline, newline)),
    )(i)
}

struct RangesMap<'a> {
    ranges: Vec<(u64, &'a str, u64, &'a str, u64)>,
}

impl<'a> RangesMap<'a> {
    fn map(&self, n: u64) -> u64 {
        for &(dst, _, src, _, len) in &self.ranges {
            if (src..(src + len)).contains(&n) {
                return n - src + dst;
            }
        }
        n
    }
}

fn parse_map(i: &str) -> IResult<&str, RangesMap> {
    let (rem, _) = tuple((alpha1, tag("-to-"), alpha1, tag(" map:\n")))(i)?;
    map(
        separated_list1(newline, tuple((u64, space1, u64, space1, u64))),
        |ranges| RangesMap { ranges },
    )(rem)
}

fn parse_maps(i: &str) -> IResult<&str, Vec<RangesMap>> {
    separated_list1(tuple((newline, newline)), parse_map)(i)
}

fn solution(s: &str) -> u64 {
    let (_, (mut seeds, maps)) = tuple((parse_seeds, parse_maps))(s).unwrap();

    for map in maps {
        for seed in &mut seeds {
            *seed = map.map(*seed);
        }
    }
    *seeds.iter().min().unwrap()
}

#[test]
fn part_1() {
    const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(35, solution(TEST));
}
