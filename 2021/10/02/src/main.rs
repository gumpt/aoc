use phf::phf_map;

static OPEN_TO_CLOSED: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

static CLOSED_TO_OPEN: phf::Map<char, char> = phf_map! {
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
};

static SCORE: phf::Map<char, u64> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
};

fn get_line_completions(line: &str) -> Option<String> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if CLOSED_TO_OPEN.contains_key(&c) {
            if stack.pop().unwrap() != CLOSED_TO_OPEN[&c] {
                return None;
            }
            continue;
        }
        stack.push(c);
    }
    Some(stack.iter().rev().map(|c| OPEN_TO_CLOSED[&c]).collect())
}

fn score(incompletion: &str) -> u64 {
    incompletion.chars().fold(0, |acc, c| acc * 5 + SCORE[&c])
}

fn main() {
    let input = include_str!("../input.txt");

    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|line| get_line_completions(line))
        .map(|incompletion| score(&incompletion))
        .collect();
    scores.sort();
    println!("PART 2: {:?}", scores[scores.len() / 2]);
}
