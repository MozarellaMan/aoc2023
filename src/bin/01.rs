use core::num;

advent_of_code::solution!(1);

fn get_first_and_last_numbers(line: &str) -> (char, char) {
    let numbers: Vec<char> = line
        .as_bytes()
        .iter()
        .filter(|x| x.is_ascii_digit())
        .map(|x| *x as char)
        .collect();

    (*numbers.first().unwrap(), *numbers.last().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (first, last) = get_first_and_last_numbers(line);
        let mut number = String::new();
        number.push(first);
        number.push(last);

        let number = u32::from_str_radix(&number, 10).expect("invalid number!");

        sum += number;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
