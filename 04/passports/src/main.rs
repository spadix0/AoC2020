use std::{io, collections::{HashMap, HashSet}};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

type Entry = HashMap<String, String>;


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let entries = read(&mut std::fs::File::open(path).unwrap());
    println!("{:?}", entries);

    println!("part[1]: {}", count_valid(&entries, validate_keys));
    println!("part[2]: {}", count_valid(&entries, validate_entry));
}


fn count_valid<P>(entries: &Vec<Entry>, validator: P) -> usize
    where P: Fn(&Entry) -> bool
{
    entries.iter()
        .cloned()
        .filter(validator)
        .count()
}


fn validate_keys(entry: &Entry) -> bool {
    lazy_static! {
        static ref REQ_FIELDS: HashSet<&'static str> =
            "byr iyr eyr hgt hcl ecl pid"
            .split_whitespace().collect();
    }
    REQ_FIELDS.iter()
        .all(|k| entry.contains_key(&**k))
}


fn validate_entry(entry: &Entry) -> bool {
    validate_keys(entry) &&
        entry.iter()
            .all(|(k, v)| validate_field(k, v))
}


fn validate_field(k: &str, v: &str) -> bool {
    lazy_static! {
        static ref EYE_COLORS: HashSet<&'static str> =
            "amb blu brn gry grn hzl oth"
            .split_whitespace().collect();

        static ref HGT_RE: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    match k {
        "byr" => v.parse().map_or(false, |n| 1920 <= n && n <= 2002),
        "iyr" => v.parse().map_or(false, |n| 2010 <= n && n <= 2020),
        "eyr" => v.parse().map_or(false, |n| 2020 <= n && n <= 2030),
        "hgt" => HGT_RE.captures(v)
            .map_or(false, |m| {
                match (m[1].parse(), &m[2]) {
                    (Ok(n), "cm") => 150 <= n && n <= 193,
                    (Ok(n), "in") => 59 <= n && n <= 76,
                    _ => false,
                }
            }),
        "hcl" => HCL_RE.is_match(v),
        "ecl" => EYE_COLORS.contains(v),
        "pid" => PID_RE.is_match(v),
        "cid" => true,
        _ => false
    }
}


fn read(stm: &mut impl io::Read) -> Vec<Entry> {
    use io::BufRead;
    let mut entries = Vec::new();
    let mut in_entry = false;

    for line in io::BufReader::new(stm).lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            in_entry = false;
        }
        else if !in_entry {
            in_entry = true;
            entries.push(Entry::new())
        }

        if in_entry {
            entries.last_mut().unwrap().extend(
                line.split_whitespace()
                    .map(|field| {
                        let mut toks = field.split(':');
                        (toks.next().unwrap().into(),
                         toks.next().unwrap().into())
                    })
            );
        }
    }

    entries
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_cases() {
        for (ent, &exp) in read_ex0().iter().zip(&[
            true, false, true, false
        ]) {
            assert_eq!(exp, validate_keys(ent));
        }
    }

    #[test]
    fn ex0_count() {
        assert_eq!(2, count_valid(&read_ex0(), validate_keys));
    }

    #[test]
    fn ex_fields() {
        for (key, val, exp) in EX_FIELDS {
            assert_eq!(*exp, validate_field(key, val));
        }
    }

    #[test]
    fn ex1_invalid() {
        for ent in read(&mut EX1_INVALID.as_bytes()) {
            assert!(!validate_entry(&ent));
        }
    }

    #[test]
    fn ex2_valid() {
        for ent in read(&mut EX2_VALID.as_bytes()) {
            assert!(validate_entry(&ent));
        }
    }

    fn read_ex0() -> Vec<Entry> {
        read(&mut EX0.as_bytes())
    }

    const EX0: &str = include_str!("../../ex0.txt");
    const EX1_INVALID: &str = include_str!("../../ex1_invalid.txt");
    const EX2_VALID: &str = include_str!("../../ex2_valid.txt");

    const EX_FIELDS: &[(&str, &str, bool)] = &[
        ("byr", "2002", true),
        ("byr", "2003", false),

        ("hgt", "60in", true),
        ("hgt", "190cm", true),
        ("hgt", "190in", false),
        ("hgt", "190", false),

        ("hcl", "#123abc", true),
        ("hcl", "#123abz", false),
        ("hcl", "123abc", false),

        ("ecl", "brn", true),
        ("ecl", "wat", false),

        ("pid", "000000001", true),
        ("pid", "0123456789", false),
    ];
}
