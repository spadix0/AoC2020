use std::{
    mem::swap,
    iter::repeat,
};
use super::{DIRS, Seats};

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<u8>,
    edges: Vec<u32>,
}


impl Graph {
    pub fn adjacent(seats: &Seats) -> Graph {
        let (inode, n) = compress_seats(seats);
        let mut edges = Vec::with_capacity(n*8);
        let w = seats.width as i32;
        let dirs = [ -w-1, -w, -w+1, -1, 1, w-1, w, w+1 ];

        let nodes = inode.iter()
            .enumerate()
            .filter(|(_, &i)| i >= 0)
            .map(|(p, _)| {
                let p = p as i32;
                let n0 = edges.len();
                edges.extend(
                    dirs.iter()
                        .filter_map(|dp| match inode[(p + dp) as usize] {
                            q if q >= 0 => Some(q as u32),
                            _ => None
                        })
                );
                (edges.len() - n0) as u8
            })
            .collect();

        edges.shrink_to_fit();

        Graph { nodes, edges }
    }

    pub fn visible(seats: &Seats) -> Graph {
        let (inode, n) = compress_seats(seats);
        let mut nodes = Vec::with_capacity(n);
        let mut edges = Vec::with_capacity(n*8);
        let w = seats.width as i32;
        let h = seats.grid.len() as i32 / w;

        let search_line = |mut x, mut y, (dx, dy)| {
            x += dx;
            y += dy;
            while 0 <= x && x < w && 0 <= y && y < h {
                let j = inode[(w*y + x) as usize];
                if j >= 0 {
                    return Some(j as u32);
                }
                x += dx;
                y += dy;
            }
            None
        };

        for y in 0..h {
            for x in 0..w {
                let p = (w*y + x) as usize;
                let i = inode[p];
                if i >= 0 {
                    let n0 = edges.len();
                    edges.extend(
                        DIRS.iter().filter_map(|&dp| search_line(x, y, dp))
                    );
                    assert_eq!(i as usize, nodes.len());
                    nodes.push((edges.len() - n0) as u8);
                }
            }
        }

        edges.shrink_to_fit();

        Graph { nodes, edges }
    }

    pub fn run_until_stable(&self, thresh: u8) -> Vec<u8> {
        let n = self.nodes.len();
        let mut prev: Vec<_> = repeat(1).take(n).collect();
        let mut next: Vec<_> = repeat(0).take(n).collect();

        while next != prev {
            swap(&mut next, &mut prev);
            self.step(&prev, &mut next, thresh);
        }
        next
    }

    fn step(&self, prev: &[u8], next: &mut [u8], thresh: u8) {
        let n = self.nodes.len();
        let nodes = &self.nodes[..n];
        let prev = &prev[..n];
        let next = &mut next[..n];

        for i in 0..n { next[i] = 0; }

        let mut j: usize = 0;
        for i in 0..n {
            let j1 = j + nodes[i] as usize;
            if prev[i] != 0 {
                for k in j..j1 {
                    unsafe { // ~20% faster :|
                        *next.get_unchecked_mut(
                            *self.edges.get_unchecked(k) as usize
                        ) += 1;
                    }
                }
            }
            j = j1;
        }

        for i in 0..n {
            next[i] = if prev[i] == 0 {
                next[i] == 0
            } else {
                next[i] < thresh
            } as u8;
        }
    }
}


fn compress_seats(seats: &Seats) -> (Vec<i32>, usize) {
    let mut inode = 0..;
    (
        seats.grid.iter()
            .map(|&s| if s { inode.next().unwrap() } else { -1 })
            .collect(),
        inode.next().unwrap() as usize,
    )
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{*, tests::*};

    fn check_graph(graph: &Graph, thresh: u8, exp: u32) {
        let occ = graph.run_until_stable(thresh);
        assert_eq!(count_occupied(&occ), exp);
    }

    #[test]
    fn ex0_adjacent() {
        let seats = Seats::read(&mut EX0.as_bytes());
        check_graph(&Graph::adjacent(&seats), 4, 37);
    }

    #[test]
    fn answer1() {
        let seats = Seats::read(&mut INPUT.as_bytes());
        check_graph(&Graph::adjacent(&seats), 4, 2368);
    }

    #[test]
    fn ex0_visible() {
        let seats = Seats::read(&mut EX0.as_bytes());
        check_graph(&Graph::visible(&seats), 5, 26);
    }

    #[test]
    fn answer2() {
        let seats = Seats::read(&mut INPUT.as_bytes());
        check_graph(&Graph::visible(&seats), 5, 2124);
    }
}
