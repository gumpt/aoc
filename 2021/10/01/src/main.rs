use phf::phf_map;

static MATCHING: phf::Map<char, char> = phf_map! {
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
};

static SCORE: phf::Map<char, u32> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};

fn score_line(line: &str) -> u32 {
    let mut stack = Vec::new();
    for c in line.chars() {
        if MATCHING.contains_key(&c) {
            if stack.pop().unwrap() != MATCHING[&c] {
                return SCORE[&c];
            }
            continue;
        }
        stack.push(c);
    }
    return 0;
}

fn main() {
    let input = include_str!("../input.txt").lines();

    let scores = input.fold(0, |acc, line| acc + score_line(line));
    println!("PART 1: {}", scores);
}
