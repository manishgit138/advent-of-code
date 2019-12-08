use crate::int_code::Program;

pub fn part1(input_string: &str) -> String {
    let mut program = Program::parse(input_string);
    program.input(1);
    program.run();
    program.output_values.last().unwrap().to_string()
}

pub fn part2(input_string: &str) -> String {
    let mut program = Program::parse(input_string);
    program.input(5);
    program.run();
    program.output_values.last().unwrap().to_string()
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day05_input.txt")), "15097178");
}

#[test]
fn tests_part2() {
    assert_eq!(part2(include_str!("day05_input.txt")), "1558663");
}