use std::{io, collections::HashMap};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let data = read(&mut std::fs::File::open(path).unwrap());
    //println!("{:?}", data);

    let (a, b) = find_sum2(2020, &data).unwrap();
    println!("part[1]: {} * {} = {}", a, b, a*b);

    let (a, b, c) = find_sum3(2020, &data).unwrap();
    println!("part[2]: {} * {} * {} = {}", a, b, c, a*b*c);
}


fn read(stream: &mut impl io::Read) -> Vec<i32> {
    use io::BufRead;
    io::BufReader::new(stream)
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
}

fn index_by_value(data: &[i32]) -> HashMap<i32, usize> {
    data.iter()
        .enumerate()
        .map(|(i,&x)| (x,i))
        .collect()
}

fn find_sum2(sum: i32, data: &[i32]) -> Option<(i32, i32)> {
    let lut = index_by_value(data);
    data.iter().cloned()
        .enumerate()
        .find_map(|(i, a)| {
            let b = sum - a;
            // FIXME still experimental:
            // (*lut.get(&b)? > i).then_some((a, b))
            if *lut.get(&b)? > i {
                Some((a, b))
            } else {
                None
            }
        })
}

fn find_sum3(sum: i32, data: &[i32]) -> Option<(i32, i32, i32)> {
    data.iter().cloned()
        .enumerate()
        .find_map(|(i, a)| {
            // this one just reuses sum2, which recreates index
            // for each outer iteration (but still linear)
            if let Some((b, c)) = find_sum2(sum-a, &data[i+1..]) {
                Some((a, b, c))
            } else {
                None
            }
        })
}

// manual loop for comparison
#[allow(dead_code)]
fn find_sum3_loop(sum: i32, data: &[i32]) -> Option<(i32, i32, i32)> {
    let lut = index_by_value(data);
    for (i, &a) in data.iter().enumerate() {
        for (j, &b) in data[i+1..].iter().enumerate() {
            let c = sum - a - b;
            if let Some(k) = lut.get(&c) {
                if *k > i + 1 + j {
                    return Some((a, b, c));
                }
            }
        }
    }
    None
}

// recursive to handle arbitrary number of elements to sum
// reuses index, O(len(data)^(n-1))
#[allow(dead_code)]
fn find_sum_rec(n: usize, sum: i32, data: &[i32]) -> Option<Vec<i32>> {
    struct Env<'a> {
        data: &'a [i32],
        lut: HashMap<i32, usize>,
    }

    fn finder(e: &Env, n: usize, i0: usize, sum: i32) -> Option<Vec<i32>> {
        if n == 1 {
            if let Some(&i) = e.lut.get(&sum) {
                if i >= i0 {
                    return Some(vec![sum]);
                }
            }
        }
        else if e.data.len() >= i0 + n {
            for (i, &a) in e.data[i0..].iter().enumerate() {
                if let Some(mut v) = finder(e, n-1, i0+i+1, sum-a) {
                    v.push(a);
                    return Some(v);
                }
            }
        }
        None
    };

    finder(&Env{data, lut: index_by_value(data)}, n, 0, sum)
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        assert_eq!(read_ex0(), [1721, 979, 366, 299, 675, 1456]);
    }

    #[test]
    fn sum2_ex0() {
        check_sum2(2020, &read_ex0(), (1721, 299))
    }

    #[test]
    fn sum3_ex0() {
        check_sum3(2020, &read_ex0(), (979, 366, 675));
    }

    // same element should not be considered multiple times
    #[test]
    fn sum2_half_nodup() {
        check_sum2(2020, &[ 1010, 1009, 1011 ], (1009, 1011));
    }

    #[test]
    fn sum3_third_nodup() {
        check_sum3(2019, &[ 673, 1, 2, 672, 674, 3 ], (673, 672, 674));
    }

    #[test]
    fn sum3_half_nodup() {
        check_sum3(2020, &[ 674, 673, 979, 366, 675 ], (979, 366, 675));
    }

    // identical elements should be considered for each instance
    #[test]
    fn sum2_dup2() {
        check_sum2(2020, &[ 1, 1010, 2, 1010, 3 ], (1010, 1010));
    }

    #[test]
    fn sum3_dup2() {
        check_sum3(2020, &[ 674, 1, 673, 2, 673, 3 ], (674, 673, 673));
    }

    #[test]
    fn sum3_dup3() {
        check_sum3(2019, &[ 673, 1, 673, 2, 673, 3 ], (673, 673, 673));
    }


    fn check_sum2(sum: i32, data: &[i32], exp: (i32, i32)) {
        assert_eq!(Some(exp), find_sum2(sum, data));
        assert_eq!(Some(vec![exp.1, exp.0]), find_sum_rec(2, sum, data))
    }

    fn check_sum3(sum: i32, data: &[i32], exp: (i32, i32, i32)) {
        assert_eq!(Some(exp), find_sum3(sum, data));
        assert_eq!(Some(exp), find_sum3_loop(sum, data));
        assert_eq!(Some(vec![exp.2, exp.1, exp.0]), find_sum_rec(3, sum, data))
    }

    fn read_ex0() -> Vec<i32> {
        return super::read(&mut EX0.as_bytes())
    }

    const EX0: &str = "\
1721
979
366
299
675
1456
";
}
