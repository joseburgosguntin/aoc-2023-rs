use nom::character::complete::{space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{bytes::complete::tag, IResult};

fn main() {
    let text = include_str!("./day_4.txt").trim();
    println!("{}", solution(text));
}

fn parse_card_id(i: &str) -> IResult<&str, u32> {
    preceded(preceded(tag("Card"), space1), u32)(i)
}

fn parse_space_numbers(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(i)
}

fn have_win_numbers(i: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(
        parse_space_numbers,
        delimited(space1, tag("|"), space1),
        parse_space_numbers,
    )(i)
}

fn line_parser(i: &str) -> IResult<&str, (u32, (Vec<u32>, Vec<u32>))> {
    separated_pair(
        parse_card_id,
        terminated(tag(":"), space1),
        have_win_numbers,
    )(i)
}

fn solution(s: &str) -> u32 {
    let mut res = 0;
    for line in s.split('\n') {
        let (_, (_, (have, win))) = line_parser(dbg!(line)).unwrap();
        res += match have.into_iter().filter(|x| win.contains(x)).count() {
            0 => 0,
            x => 2u32.pow(x as u32 - 1),
        };
    }
    res
}

#[test]
fn part_1() {
    const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(13, solution(TEST));
}
