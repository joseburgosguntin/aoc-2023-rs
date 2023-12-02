use nom::branch::alt;
use nom::combinator::map;
use nom::{bytes::complete::tag, IResult};

fn main() {
    let text = include_str!("./day_1.txt");
    println!("{}", solution(text));
}

fn word_to_u32(word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

fn parse_word(i: &str) -> IResult<&str, u32> {
    map(
        alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
        )),
        |word| word_to_u32(word),
    )(i)
}

fn parse_single_digit(i: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(&i[..1])
}

fn parse_num(i: &str) -> IResult<&str, u32> {
    alt((parse_word, parse_single_digit))(i)
}

fn my_combinator<O>(p: impl Fn(&str) -> IResult<&str, O>, i: &str) -> Vec<O> {
    (0..i.len())
        .filter_map(|start| p(&i[start..]).map(|(_, o)| o).ok())
        .collect()
}

fn solution(s: &str) -> u32 {
    let mut res = 0;
    for line in s.split('\n') {
        // dbg!(my_combinator(parse_num, line));
        let mut nums = my_combinator(parse_num, line).into_iter();

        let first = nums.next().unwrap_or(0);
        res += first * 10;
        res += nums.last().unwrap_or(first);
    }
    res
}

#[test]
fn part_2() {
    const TEST: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(281, solution(TEST));
}
