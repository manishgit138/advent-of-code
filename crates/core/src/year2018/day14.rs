pub fn part1(input_string: &str) -> Result<String, String> {
    let input_number = input_string
        .parse::<usize>()
        .map_err(|error| format!("Invalid input: {}", error.to_string()))?;
    let desired_length = input_number + 10;

    let mut scores = vec![3_u8, 7_u8];
    let mut elf_positions = vec![0, 1];

    while scores.len() < desired_length {
        let current_recipes_score = scores[elf_positions[0]] + scores[elf_positions[1]];

        if current_recipes_score < 10 {
            scores.push(current_recipes_score);
        } else {
            scores.push(current_recipes_score / 10);
            scores.push(current_recipes_score % 10);
        }

        for position in elf_positions.iter_mut() {
            *position = (*position + 1 + scores[*position as usize] as usize) % scores.len();
        }
    }

    Ok(scores
        .iter()
        .skip(input_number)
        .take(10)
        .fold(String::new(), |acc, score| acc + &score.to_string()))
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    let input_bytes: Vec<u8> = input_string
        .chars()
        .map(|b| {
            b.to_digit(10)
                .map(|b| b as u8)
                .ok_or_else(|| "Invalid input".to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;

    let mut scores = vec![3_u8, 7_u8];
    let mut elf_positions = vec![0, 1];

    loop {
        let current_recipes_score = scores[elf_positions[0]] + scores[elf_positions[1]];

        let scores_to_push = if current_recipes_score < 10 {
            vec![current_recipes_score]
        } else {
            vec![current_recipes_score / 10, current_recipes_score % 10]
        };

        for score in scores_to_push {
            scores.push(score);
            if scores.ends_with(&input_bytes) {
                return Ok(scores.len() - input_string.len());
            }
        }

        for position in elf_positions.iter_mut() {
            *position = (*position + 1 + scores[*position as usize] as usize) % scores.len();
        }
    }
}

#[test]
fn tests_part1() {
    assert_eq!(Ok("5158916779".to_string()), part1("9"));
    assert_eq!(Ok("0124515891".to_string()), part1("5"));
    assert_eq!(Ok("9251071085".to_string()), part1("18"));
    assert_eq!(Ok("5941429882".to_string()), part1("2018"));

    assert_eq!(
        Ok("1150511382".to_string()),
        part1(include_str!("day14_input.txt"))
    );
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(9), part2("51589"));
    assert_eq!(Ok(5), part2("01245"));
    assert_eq!(Ok(18), part2("92510"));
    assert_eq!(Ok(2018), part2("59414"));

    assert_eq!(Ok(20_173_656), part2(include_str!("day14_input.txt")));
}
