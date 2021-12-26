#[derive(Debug)]
struct Line {
    input: Vec<String>,
    output: Vec<String>,
}

fn find_from_predicate<'a, F>(candidates: &'a Vec<String>, predicate: F) -> String
where
    F: FnMut(&&String) -> bool,
{
    candidates.iter().find(predicate).unwrap().to_owned()
}

fn contains(outer: &str, inner: &str) -> bool {
    inner.chars().all(|c| outer.contains(c))
}

impl Line {
    fn sanitize(input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .map(|w| {
                let mut chars: Vec<char> = w.chars().collect();
                chars.sort();
                chars.iter().collect::<String>()
            })
            .collect()
    }

    fn new((input, output): (&str, &str)) -> Line {
        Line {
            input: Line::sanitize(input),
            output: Line::sanitize(output),
        }
    }

    fn input_of_length(&self, len: usize) -> String {
        self.inputs_of_length(len).first().unwrap().to_string()
    }

    fn inputs_of_length(&self, len: usize) -> Vec<String> {
        self.input
            .iter()
            .filter_map(|i| {
                if i.len() == len {
                    Some(i.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    }

    fn solve(&self) -> usize {
        let mut decoder = vec!["".to_owned(); 10];
        decoder[8] = self.input_of_length(7); // abcdefg
        decoder[1] = self.input_of_length(2); // cf
        decoder[4] = self.input_of_length(4); // bcdf
        decoder[7] = self.input_of_length(3); // acf

        // 2, 3, 5
        let two_three_five = self.inputs_of_length(5);
        let zero_six_nine = self.inputs_of_length(6);
        decoder[3] = find_from_predicate(&two_three_five, |symbol| contains(symbol, &decoder[1])); // acdfg
        decoder[9] = find_from_predicate(&zero_six_nine, |symbol| contains(symbol, &decoder[4])); // abcdfg

        decoder[6] = find_from_predicate(&zero_six_nine, |symbol| !contains(symbol, &decoder[1])); // abdefg
        decoder[5] = find_from_predicate(&two_three_five, |symbol| contains(&decoder[6], symbol)); // abdfg

        // I give up
        decoder[0] = find_from_predicate(&zero_six_nine, |symbol| {
            *symbol != &decoder[6] && *symbol != &decoder[9]
        }); // abcefg
        decoder[2] = find_from_predicate(&two_three_five, |symbol| {
            *symbol != &decoder[3] && *symbol != &decoder[5]
        }); // acdeg

        // decoder
        self.output.iter().fold(0, |acc, word| {
            acc * 10 + decoder.iter().position(|key| word == key).unwrap()
        })
    }
}

fn main() {
    // let input =
    // "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let input = include_str!("../input.txt");

    let lines: Vec<Line> = input
        .lines()
        .map(|line| {
            let parts = line
                .split('|')
                .map(|part| part.trim())
                .collect::<Vec<&str>>();
            Line::new((parts[0], parts[1]))
        })
        .collect();

    println!("{}", lines.iter().fold(0, |acc, l| acc + l.solve()));

    // let f = outputs.iter().fold(0, |acc, output| {
    //     let parts = output.split_whitespace().fold(0, |mut acc, word| {
    //         acc + match word.len() {
    //             2 | 3 | 4 | 7 => 1,
    //             _ => 0,
    //         }
    //     });
    //     acc + parts
    // });
    // println!("Part 1: {:?}", f);
}
