use std::cmp::Ordering;

fn parse_input(input_string: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = input_string.trim().split('-').collect();
    if parts.len() != 2 {
        return Err("Invalid parts - should be in FROM-TO form".to_string());
    }
    let from = parts[0].parse::<i32>().or(Err("Invalid range"))?;
    let to = parts[1].parse::<i32>().or(Err("Invalid range"))?;
    Ok((from, to))
}

fn solution(input_string: &str, part1: bool) -> Result<u32, String> {
    let (from, to) = parse_input(input_string)?;

    let mut meeting_criteria_count: u32 = 0;
    'outer: for i in from..=to {
        let mut divider = 1;
        let mut last_digit = 10;
        let mut two_digits_adjacent = false;

        let mut digits_adjacent_streak = 1;

        while divider <= 100_000 {
            let digit = (i / divider) % 10;
            match digit.cmp(&last_digit) {
                Ordering::Greater => {
                    continue 'outer;
                }
                Ordering::Equal => {
                    if part1 {
                        two_digits_adjacent = true;
                    } else {
                        digits_adjacent_streak += 1;
                    }
                }
                Ordering::Less => {
                    if digits_adjacent_streak == 2 {
                        two_digits_adjacent = true;
                    }
                    digits_adjacent_streak = 1;
                }
            }

            last_digit = digit;
            divider *= 10;
        }

        if digits_adjacent_streak == 2 {
            two_digits_adjacent = true;
        }

        if two_digits_adjacent {
            meeting_criteria_count += 1;
        }
    }

    Ok(meeting_criteria_count)
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, false)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1("100010-100011"), Ok(0));
    assert_eq!(part1("111110-111111"), Ok(1));

    assert_eq!(part1(include_str!("day04_input.txt")), Ok(1675));
}

#[test]
fn tests_part2() {
    assert_eq!(part2("112233-112233"), Ok(1));
    assert_eq!(part2("123444-123444"), Ok(0));
    assert_eq!(part2("111122-111122"), Ok(1));

    assert_eq!(part2(include_str!("day04_input.txt")), Ok(1142));
}
