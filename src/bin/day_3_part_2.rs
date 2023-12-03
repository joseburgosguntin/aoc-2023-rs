use std::collections::HashMap;

fn main() {
    let text = include_str!("./day_3.txt");
    println!("{}", solution(text));
}

fn solution(s: &str) -> u32 {
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();
    let vec = s.split('\n').collect::<Vec<_>>();
    let mut gear = None;
    let mut num = 0;
    for (i, &line) in vec.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match (char, gear) {
                ('0'..='9', _) => {
                    num *= 10;
                    num += char.to_digit(10).unwrap();
                    let arr = [
                        (1, 1),
                        (1, 0),
                        (1, -1),
                        (0, 1),
                        (0, -1),
                        (-1, 1),
                        (-1, 0),
                        (-1, -1),
                    ];
                    if let None = gear {
                        for (x, y) in arr {
                            let coord = match (i as i32 + x, j as i32 + y) {
                                (i @ 0.., j @ 0..) => Some((i as usize, j as usize)),
                                _ => None,
                            };
                            coord
                                .and_then(|(i, j)| vec.get(i)?.as_bytes().get(j))
                                .map(|&char| match char as char {
                                    '*' => gear = coord,
                                    _ => {}
                                });
                        }
                    }
                }
                (_, Some(coord)) => {
                    gears
                        .entry(coord)
                        .and_modify(|v| v.push(num))
                        .or_insert(vec![num]);
                    num = 0;
                    gear = None;
                }
                _ => {
                    num = 0;
                    gear = None;
                }
            }
        }
    }
    gears
        .into_iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.into_iter().product::<u32>())
        .sum()
}

#[test]
fn part_2() {
    const TEST: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(467835, solution(TEST));
}
