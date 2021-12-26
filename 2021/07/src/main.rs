fn main() {
    let mut numbers = include_str!("../input.txt")
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    numbers.sort();

    let mut mids: Vec<usize> = vec![numbers.len() / 2];
    if numbers.len() % 2 == 0 {
        mids.push((((numbers.len() as f64 / 2f64).floor()) as usize) - 1);
    }

    let values = mids.iter().map(|&n| numbers[n]).collect::<Vec<_>>();
    let sum: usize = values.iter().sum();
    let median = sum as f64 / mids.len() as f64;

    // println!(
    //     "{:?} ({}): => {:?} => {:?} => {}",
    //     numbers,
    //     numbers.len(),
    //     mids,
    //     values,
    //     median
    // );

    let mut fuel: f64 = 0.0;
    for n in numbers.iter() {
        fuel += (*n as f64 - median).abs();
    }
    println!("Part 1: {}", fuel);

    let mean: usize = numbers.iter().sum::<usize>() / numbers.len();
    fuel = 0.0;
    for n in numbers.iter() {
        let distance = (*n as f64 - mean as f64).abs();
        fuel += (distance * (distance + 1.0)) as f64 / 2.0;
    }
    println!("Part 2: {}", fuel);
}
