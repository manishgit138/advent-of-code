use crate::input::Input;

pub fn solve(_input: &mut Input) -> Result<u32, String> {
    Ok(0)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 0);
    test_part_two!(real_input => 0);
}