enum MoreOf {
    Zeroes,
    Ones,
}

fn more_of_what_in_column(lines: &[&str], column: usize) -> MoreOf {
    let ones_count = lines.iter().fold(0, |rval, &line| {
        rval + match line.chars().nth(column).unwrap() {
            '1' => 1,
            '0' => 0,
            _ => unreachable!(),
        }
    });
    let zeroes_count = lines.iter().fold(0, |rval, &line| {
        rval + match line.chars().nth(column).unwrap() {
            '1' => 0,
            '0' => 1,
            _ => unreachable!(),
        }
    });
    if zeroes_count > ones_count {
        return MoreOf::Zeroes;
    }
    return MoreOf::Ones;
}

fn oxygen_rating(lines: &[&str]) -> i64 {
    let mut candidates: Vec<&str> = lines.to_vec();
    let mut column = 0;
    loop {
        let nth = match more_of_what_in_column(&candidates, column) {
            MoreOf::Zeroes => '0',
            MoreOf::Ones => '1',
            _ => unreachable!(),
        };
        candidates.retain(|line| line.chars().nth(column).unwrap() == nth);
        if candidates.len() == 1 {
            break;
        }
        column += 1;
    }
    return i64::from_str_radix(&candidates[0], 2).unwrap();
}

fn dioxide_scrubber(lines: &[&str]) -> i64 {
    let mut candidates = lines.to_vec();
    let mut column = 0;
    loop {
        let nth = match more_of_what_in_column(&candidates, column) {
            MoreOf::Zeroes => '1',
            MoreOf::Ones => '0',
            _ => unreachable!(),
        };
        candidates.retain(|line| line.chars().nth(column).unwrap() == nth);
        if candidates.len() == 1 {
            break;
        }
        column += 1;
    }
    return i64::from_str_radix(&candidates[0], 2).unwrap();
}

fn main() {
    let grid: Vec<_> = include_str!("../input.txt").lines().collect();

    println!(
        "{} {} {}",
        oxygen_rating(&grid),
        dioxide_scrubber(&grid),
        oxygen_rating(&grid) * dioxide_scrubber(&grid)
    );
}
