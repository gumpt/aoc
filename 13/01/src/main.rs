use text_io::read;

fn main() {
    let departure_time: usize = read!();

    let second_line: String = read!();
    let ids = second_line
        .split(",")
        .filter(|id| id != &"x")
        .map(|id| id.parse::<usize>())
        .filter_map(Option::Some)
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();

    let mut target = departure_time;
    let mut result = None;
    while None == result {
        for id in ids.iter() {
            if target % id == 0 {
                result = Some((target - departure_time) * id);
            }
        }
        target += 1;
    }

    println!("PART 1: {}", result.unwrap());
}
