use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

const M: u64 = 20201227;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let pubkey = parse(&std::fs::read_to_string(path).unwrap());
    //println!("{:?}", pubkey);

    let loop0 = reverse_loop(7, pubkey[0]);
    println!("card loop size: {}", loop0);

    let loop1 = reverse_loop(7, pubkey[1]);
    println!("door loop size: {}", loop1);

    let enckey = calc_enckey(pubkey[1], loop0);
    println!("encryption key: {}", enckey);

    assert_eq!(enckey, calc_enckey(pubkey[0], loop1));
}


fn parse(s: &str) -> Vec<u32> {
    s.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}


fn reverse_loop(subj: u32, tgt: u32) -> u32 {
    let subj = subj as u64;
    let tgt = tgt as u64;
    let mut val: u64 = 1;
    for i in 1.. {
        val = val * subj % M;
        if val == tgt { return i as u32; }
    }
    std::unreachable!();
}


fn calc_enckey(k: u32, n: u32) -> u32 {
    let k: BigInt = k.into();
    let n: BigInt = n.into();
    let m: BigInt = M.into();
    k.modpow(&n, &m).to_u32().unwrap()
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_card_loop() {
        assert_eq!(reverse_loop(7, 5764801), 8);
    }

    #[test]
    fn ex0_door_loop() {
        assert_eq!(reverse_loop(7, 17807724), 11);
    }

    #[test]
    fn ex0_key() {
        assert_eq!(calc_enckey(5764801, 11), 14897079);
        assert_eq!(calc_enckey(17807724, 8), 14897079);
    }

    #[test]
    fn answer() {
        let pubkey = parse(INPUT);
        let loop0 = reverse_loop(7, pubkey[0]);
        assert_eq!(loop0, 2232839);

        let loop1 = reverse_loop(7, pubkey[1]);
        assert_eq!(loop1, 529361);

        assert_eq!(calc_enckey(pubkey[1], loop0), 11328376);
        assert_eq!(calc_enckey(pubkey[0], loop1), 11328376);
    }

    pub const INPUT: &str = include_str!("../../input.txt");
}
