fn generation(input: &[usize]) -> [usize; 9] {
    input.iter().fold([0; 9], |mut acc, &n| {
        // println!("... {:?} {}", acc, n);
        acc[n] += 1;
        acc
    })
}

fn simulate(input: &[usize; 9]) -> [usize; 9] {
    let mut new = *input;
    new.rotate_left(1);
    new[6] += new[8];
    new
}

fn main() {
    // let starting = "3,4,3,1,2"
    let starting = include_str!("../input.txt")
        .split(',')
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut f = generation(&starting);
    for _ in 0..256 {
        println!("F: {:?}", f);
        f = simulate(&f);
    }

    println!("{}", f.into_iter().sum::<usize>());
}
