pub fn part1(input_string: &str) -> String {
    input_string
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(|mass| (mass / 3) - 2)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input_string: &str) -> String {
    input_string
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(|mass| (mass / 3) - 2)
        .map(|mut additional_fuel| {
            let mut total_fuel = 0;
            while additional_fuel > 0 {
                total_fuel += additional_fuel;
                additional_fuel = additional_fuel / 3 - 2;
            }
            total_fuel
        })
        .sum::<i32>()
        .to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!("2", part1("12"));
    assert_eq!("2", part1("14"));
    assert_eq!("654", part1("1969"));
    assert_eq!("33583", part1("100756"));

    assert_eq!("3262358", part1(include_str!("day1_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("2", part2("14"));
    assert_eq!("966", part2("1969"));
    assert_eq!("50346", part2("100756"));

    assert_eq!("4890696", part2(include_str!("day1_input.txt")));
}
