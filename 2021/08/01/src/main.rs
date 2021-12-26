fn main() {
    let input = include_str!("../input.txt");

    let outputs: Vec<String> = input
        .lines()
        .map(|line| {
            line.split('|')
                .map(|part| part.trim())
                .skip(1)
                .collect::<String>()
        })
        .collect();

    let f = outputs.iter().fold(0, |acc, output| {
        let parts = output.split_whitespace().fold(0, |acc, word| {
            acc + match word.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        });
        acc + parts
    });
    println!("Part 1: {:?}", f);
}
