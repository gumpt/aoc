use text_io::read;

/*
Stealing from Rosetta Code for the Chinese Remainder Theorem. I am not quite
big-brained enough to write this code, but have enough familiarity with crypto
to recognize a problem that it can solve.

What use it's serving: We're looking for some n where
n mod b = b - i

Where b is the bus id and i is the offset. You can pass this to the CRT and it
does all of the combining to find the right LCM that satisfies the constraints
because ~~ number theory ~~ is effectively magic.

This is also part of how attacks on private key cryptography operate in
reasonable real-world attacks.

Some formal proof: https://crypto.stanford.edu/pbc/notes/numbertheory/crt.html

Rosetta Code source:
https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
 */

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn main() {
    let _departure_time: usize = read!();

    let second_line: String = read!();
    let offsets_and_ids = second_line
        .split(",")
        .enumerate()
        .filter(|(_, id)| id != &"x")
        .map(|(i, id)| (i as i64, id.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    let mods = offsets_and_ids.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    let res = offsets_and_ids
        .iter()
        .map(|(i, b)| b - i)
        .collect::<Vec<_>>();

    println!("PART 2: {}", chinese_remainder(&res, &mods).unwrap());
}
