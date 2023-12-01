fn main() {
    let text = include_str!("./day_1.txt");
    println!("{}", solution(text));
}

fn solution(s: &str) -> u32 {
    let mut res = 0;
    for line in s.split('\n') {
        let mut two = line.chars().filter(|x| x.is_digit(10));
        let first = two.next().and_then(|c| c.to_digit(10)).unwrap_or(0);
        res += first * 10;
        res += two.last().and_then(|c| c.to_digit(10)).unwrap_or(first);
    }
    return res;
}

#[test]
fn part_1() {
    const TEST: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(142, solution(TEST));
}
