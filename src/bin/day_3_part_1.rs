fn main() {
    let text = include_str!("./day_3.txt");
    println!("{}", solution(text));
}

fn solution(s: &str) -> u32 {
    let vec = s.split('\n').collect::<Vec<_>>();
    let mut res = 0;
    let mut is_part = false;
    let mut num = 0;
    for (i, &line) in vec.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match (char, is_part) {
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
                    if !is_part {
                        for (x, y) in arr {
                            let coord = match (i as i32 + x, j as i32 + y) {
                                (i @ 0.., j @ 0..) => Some((i as usize, j as usize)),
                                _ => None,
                            };
                            is_part |= coord
                                .and_then(|(i, j)| vec.get(i)?.as_bytes().get(j))
                                .map(|&char| match char as char {
                                    '0'..='9' | '.' => false,
                                    _ => true,
                                })
                                .unwrap_or(false);
                        }
                    }
                }
                (_, true) => {
                    res += num;
                    num = 0;
                    is_part = false;
                }
                _ => {
                    num = 0;
                    is_part = false;
                }
            }
        }
    }
    res
}

#[test]
fn part_1() {
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
    assert_eq!(4361, solution(TEST));
}
