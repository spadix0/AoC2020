use std::{
    io,
    collections::{HashMap, HashSet},
};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let data = Ingredients::read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", data);

    let mut cons = data.constrain();
    //println!("{:?}", cons);

    let contam = union_all(&cons);
    println!("ingredients with allergens:");
    println!("    {:?}", contam.iter()
             .map(|&i| &data.ingredient[i])
             .collect::<HashSet<_>>());

    println!("part[1]: {}", data.count_except(&contam));

    let dig = match_(&mut cons);
    for (i, &j) in dig.iter().enumerate() {
        println!("    {} in {}", data.allergen[i], data.ingredient[j]);
    }

    println!("part[2]: {}", data.canonize(dig));
}


fn union_all(sets: &[HashSet<usize>]) -> HashSet<usize> {
    let mut u = HashSet::new();
    for s in sets {
        u.extend(s);
    }
    u
}


fn match_(cons: &mut Vec<HashSet<usize>>) -> Vec<usize> {
    let n = cons.len();
    let mut unvis: Vec<_> = (0..n).into_iter().collect();
    unvis.sort_unstable_by_key(|&i| -(cons[i].len() as isize));

    let mut dig = vec![0; n];

    while !unvis.is_empty() {
        let i = unvis.iter().rposition(|&x| cons[x].len() == 1).unwrap();
        let x = unvis.remove(i);
        let p = *cons[x].iter().next().unwrap();
        dig[x] = p;
        for &x in &unvis {
            cons[x].remove(&p);
        }
    }

    dig
}


#[derive(Debug)]
struct Ingredients {
    ingredient: Vec<String>,
    allergen: Vec<String>,
    contains: Vec<(HashSet<usize>, HashSet<usize>)>,
}

impl Ingredients {
    fn read(stm: &mut impl io::Read) -> Ingredients {
        use io::BufRead;

        let mut ingr = Names::new();
        let mut alrg = Names::new();

        let contains = io::BufReader::new(stm)
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut t = line
                    .trim_end_matches(')')
                    .split(" (contains ");
                (
                    t.next().unwrap()
                        .split_whitespace()
                        .map(|nm| ingr.request(nm))
                        .collect(),
                    t.next().unwrap()
                        .split(", ")
                        .map(|nm| alrg.request(nm))
                        .collect(),
                )
            })
            .collect();

        Ingredients {
            ingredient: ingr.byidx,
            allergen: alrg.byidx,
            contains,
        }
    }

    fn constrain(&self) -> Vec<HashSet<usize>> {
        let mut cons: HashMap<_, HashSet<_>> = HashMap::new();

        for (lhs, rhs) in &self.contains {
            for a in rhs {
                cons.entry(a)
                    .and_modify(|s| s.retain(|v| lhs.contains(v)))
                    .or_insert_with(|| lhs.clone());
            }
        }

        assert_eq!(cons.len(), self.allergen.len());
        (0..cons.len()).into_iter()
            .map(|i| cons.remove(&i).unwrap())
            .collect()
    }

    fn count_except(&self, filter: &HashSet<usize>) -> usize {
        self.contains.iter()
            .flat_map(|(i, _)| i)
            .filter(|i| !filter.contains(i))
            .count()
    }

    fn canonize(&self, dig: Vec<usize>) -> String {
        let mut ord: Vec<_> = (0..dig.len()).into_iter().collect();
        ord.sort_unstable_by_key(|&i| &self.allergen[i]);
        let ingr: Vec<_> = ord.into_iter()
            .map(|i| self.ingredient[dig[i]].as_str())
            .collect();
        ingr.join(",")
    }
}


struct Names {
    byidx: Vec<String>,
    byname: HashMap<String, usize>,
}

impl Names {
    fn new() -> Names {
        Names {
            byidx: Vec::new(),
            byname: HashMap::new(),
        }
    }

    fn request(&mut self, name: &str) -> usize {
        if let Some(&i) = self.byname.get(name) {
            i
        } else {
            let i = self.byidx.len();
            self.byidx.push(name.into());
            self.byname.insert(name.into(), i);
            i
        }
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex0_safe() {
        let data = Ingredients::read(&mut EX0.as_bytes());
        let contam = union_all(&data.constrain());
        assert_eq!(contam, [0, 2, 5].iter().cloned().collect());
        assert_eq!(data.count_except(&contam), 5);
    }

    #[test]
    fn answer1() {
        let data = Ingredients::read(&mut INPUT.as_bytes());
        assert_eq!(data.count_except(&union_all(&data.constrain())), 2786);
    }

    #[test]
    fn ex0_match() {
        let data = Ingredients::read(&mut EX0.as_bytes());
        let dig = match_(&mut data.constrain());
        assert_eq!(dig, &[0, 2, 5]);
        assert_eq!(data.canonize(dig), "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    fn answer2() {
        let data = Ingredients::read(&mut INPUT.as_bytes());
        assert_eq!(
            data.canonize(match_(&mut data.constrain())),
            "prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq",
        );
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
