use nom::branch::alt;
use nom::character::complete::u32;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::{bytes::complete::tag, IResult};

fn main() {
    let text = include_str!("./day_2.txt").trim();
    println!("{}", solution(text));
}

fn parse_game_id(i: &str) -> IResult<&str, u32> {
    preceded(tag("Game "), u32)(i)
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn word_to_color(word: &str) -> Color {
    match word {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    }
}

fn parse_color(i: &str) -> IResult<&str, Color> {
    map(alt((tag("red"), tag("green"), tag("blue"))), word_to_color)(i)
}

type Set = Vec<(u32, Color)>;

fn parse_set(i: &str) -> IResult<&str, Set> {
    separated_list1(tag(", "), separated_pair(u32, tag(" "), parse_color))(i)
}

fn parse_sets(i: &str) -> IResult<&str, Vec<Set>> {
    separated_list1(tag("; "), parse_set)(i)
}

fn line_parser(i: &str) -> IResult<&str, (u32, Vec<Set>)> {
    separated_pair(parse_game_id, tag(": "), parse_sets)(i)
}

fn solution(s: &str) -> u32 {
    let mut res = 0;
    for line in s.split('\n') {
        let (_, (_, sets)) = line_parser(dbg!(line)).unwrap();
        let (red, green, blue) = sets.into_iter().fold((0, 0, 0), |(red, green, blue), set| {
            let (red_i, green_i, blue_i) = set.into_iter().fold(
                (0, 0, 0),
                |(mut red, mut green, mut blue), (amount, color)| {
                    match color {
                        Color::Red => red = red.max(amount),
                        Color::Green => green = green.max(amount),
                        Color::Blue => blue = blue.max(amount),
                    }
                    (red, green, blue)
                },
            );
            (red.max(red_i), green.max(green_i), blue.max(blue_i))
        });
        res += red * green * blue;
    }
    res
}

#[test]
fn part_2() {
    const TEST: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(2286, solution(TEST));
}
