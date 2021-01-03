use std::{
    io,
    collections::HashMap,
    error::Error,
    ops::Sub,
};

type Tiles = HashMap<u32, Image>;
type TileRef = (u32, Orient);
type Matches = HashMap<(u32, i8), TileRef>;
type Coord = (usize, usize);

// FIXME these should be calculated from cute little image of fred
// *at compile time* so they remain baked constant bits
const FRED: &[Coord] = &[
    (18, 0),
    (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
    (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2),
];


fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut tiles = read(&mut std::fs::File::open(path).unwrap());

    let match_ = cache_matches(&tiles);
    let corners = find_corners(&tiles, &match_);
    println!("corners: {:?}", corners);
    println!("part[1]: {}", corners.iter().fold(1, |a, &c| a * c as u64));

    let (mosaic, size) = assemble_tiles(&match_, &corners);
    assert_eq!((size*size) as usize, mosaic.len());  // assume square

    remove_borders(&mut tiles);
    let img = Image::from_mosaic(&tiles, &mosaic, size);
    let (img, monsters) = hunt_monsters(&img);
    //println!("{:?}", monsters);
    println!("{} sea monsters", monsters.len());

    let mash = mask_monsters(&monsters, img.size());
    println!("part[2]: {}", (&img - &mash).count_roughness());
}


fn read(stm: &mut impl io::Read) -> Tiles {
    use io::BufRead;
    let mut lines = io::BufReader::new(stm).lines();
    let mut tiles = HashMap::new();
    while let Some(hdr) = lines.next() {
        let hdr = hdr.unwrap();
        assert!(hdr.starts_with("Tile"));
        let id: u32 = hdr[5..]
            .trim_end_matches(':')
            .parse().unwrap();
        tiles.insert(id, Image::read(&mut lines));
    }

    tiles
}


fn cache_matches(tiles: &Tiles) -> Matches {
    // NB assume all matched edges are unique and outside edges are unmatched
    let mut match_: Matches = Matches::new();
    let mut edges: HashMap<u16, TileRef> = HashMap::new();

    for (&id, t) in tiles {
        for dir in 0..4 {
            let e = t.edge_key(dir);
            if let Some(&(rid, ro)) = edges.get(&e) {
                match_.insert((id, dir), (rid, ro));
                match_.insert(
                    (rid, ro.dir),
                    (id, Orient { dir, flip: ro.flip })
                );
            } else {
                edges.insert(e, (id, Orient { dir, flip: true }));

                let e = (0 .. t.width).into_iter()
                    .fold(0, |a, b| a<<1 | (e>>b & 1));
                edges.insert(e, (id, Orient { dir, flip: false }));
            }
        }
    }

    match_
}


fn remove_borders(tiles: &mut Tiles) {
    for t in tiles.values_mut() {
        t.remove_border()
    }
}


fn find_corners(tiles: &Tiles, match_: &Matches) -> Vec<u32> {
    tiles.keys()
        .cloned()
        .filter(|&id| {
            // corners have exactly 2 matched edges
            2 == (0..4).into_iter()
                .filter(|&d| match_.contains_key(&(id, d)))
                .count()
        })
        .collect()
}


fn assemble_tiles(match_: &Matches, corners: &[u32]) -> (Vec<TileRef>, usize) {
    // select arbitrary corner and orient (starting at TL)
    let tlc = corners[0];
    let mt = match_.contains_key(&(tlc, 3)) as i8;
    let ml = match_.contains_key(&(tlc, 2)) as i8;
    let rot = ml<<1 | ml ^ mt;

    let mut grid = Vec::new();
    grid.push((tlc, Orient { dir: rot, flip: false }));

    let mut width = 0;
    let mut rem = match_.len() / 2;
    while rem > 0 {
        rem -= 1;

        // try to extend right
        let &(pid, po) = grid.last().unwrap();
        if let Some(&(mid, mo)) = match_.get(&(pid, po.edge(0))) {
            let next = mo.reorient(0, po.flip);
            if width > 0 {
                // up must also match
                rem -= 1;
                let (uid, uo) = grid[grid.len() - width];
                let (umid, umo) = match_[&(uid, uo.edge(1))];
                assert_eq!(mid, umid);
                assert_eq!(next, umo.reorient(1, uo.flip));
            }
            grid.push((mid, next));
        } else {
            // or extend down to next row
            if width > 0 {
                assert_eq!(grid.len() % width, 0);
            } else {
                width = grid.len();
            }
            let (uid, uo) = grid[grid.len() - width];
            let (mid, mo) = match_[&(uid, uo.edge(1))];
            let next = mo.reorient(1, uo.flip);
            grid.push((mid, next));
        }
    }

    (grid, width)
}


fn hunt_monsters(img: &Image) -> (Image, Vec<Coord>) {
    for i in 0..8 {
        let reimg = img.transformed(&Orient { dir: i>>1, flip: i&1 != 0 });
        let monsters = reimg.find_monsters();
        if monsters.len() > 0 {
            return (reimg, monsters);
        }
    }
    panic!();
}


fn mask_monsters(monsters: &[Coord], (w, h): Coord) -> Image {
    let mut img = Image {
        width: w,
        data: vec![0; w*h],
    };

    for (x0, y0) in monsters {
        for (dx, dy) in FRED {
            img.data[w*(y0+dy) + x0+dx] = 1;
        }
    }

    img
}


//----------------------------------------------------------------------------
#[derive(Copy, Clone, PartialEq, Debug)]
struct Orient {
    dir: i8,
    flip: bool,
}

impl Orient {
    // which edge to match given orientation and flip
    fn edge(&self, side: i8) -> i8 {
        if self.flip {
            self.dir-side & 3
        } else {
            side-self.dir & 3
        }
    }

    // new orientation to match adjacent tile
    fn reorient(&self, side: i8, mut flip: bool) -> Orient {
        flip ^= self.flip;
        let dir = if flip && self.dir&1 != 0 { -self.dir } else { self.dir };
        Orient { dir: 2-dir+side & 3, flip }
    }
}


#[derive(Clone, Debug)]
struct Image {
    width: usize,
    data: Vec<u8>,
}

impl Image {
    fn read(lines: &mut impl Iterator<Item=Result<String, impl Error>>)
        -> Image
    {
        let mut data = Vec::new();
        let mut width = 0;
        for line in lines {
            let line = line.unwrap();
            if line.len() == 0 {
                break;
            }

            if width == 0 {
                width = line.len();
            } else {
                assert_eq!(width, line.len());
            }

            data.extend(line.chars().map(|c| (c == '#') as u8));
        }

        Image { width: width, data }
    }

    fn size(&self) -> Coord {
        (self.width, self.data.len() / self.width)
    }

    fn edge_key(&self, dir: i8) -> u16 {
        let w = self.width;
        let data = &self.data;
        match dir & 3 {
            0 => data.iter().skip(w-1).step_by(w).pack(),
            1 => data[data.len()-w..].iter().rev().pack(),
            2 => data.iter().step_by(w).rev().pack(),
            3 => data[..w].iter().pack(),
            _ => std::unreachable!(),
        }
    }

    fn remove_border(&mut self) {
        let (w, h) = self.size();
        let (mut x, mut y) = (0, 0);

        self.data.retain(|_| {
            let keep = 0 < y && y < h-1 && 0 < x && x < w-1;
            x += 1;
            if x == w {
                x = 0;
                y += 1;
            }
            keep
        });
        self.width -= 2;
    }

    fn transformed(&self, orient: &Orient) -> Image {
        let mut data = vec![0; self.data.len()];
        let (w, h) = self.size();
        let (w, h) = (w as isize, h as isize);

        let axx = ((1 - orient.dir) % 2) as isize;
        let axy = ((orient.dir - 2) % 2) as isize;
        let ayx = ((2 - orient.dir) % 2) as isize;
        let (axy, ayy) = if orient.flip {
            (-axy, -axx)
        } else {
            (axy, axx)
        };
        let tx = (axx < 0 || axy < 0) as isize;
        let ty = (ayx < 0 || ayy < 0) as isize;
        let (tw, th) = if axx != 0 { (w, h) } else { (h, w) };

        for y in 0..h {
            for x in 0..w {
                let dx = axx*x + axy*y + tx*(tw-1);
                let dy = ayx*x + ayy*y + ty*(th-1);
                data[(tw*dy + dx) as usize] = self.data[(w*y + x) as usize];
            }
        }

        Image { data, width: tw as usize }
    }

    fn from_mosaic(tiles: &Tiles, mosaic: &[TileRef], size: usize) -> Image {
        let mut data = Vec::new();
        let mut width = 0;
        for my in 0 .. size {
            let row: Vec<_> = mosaic[my*size .. (my+1)*size].iter()
                .map(|(id, o)| tiles[id].transformed(o) )
                .collect();
            for y in 0 .. row[0].size().1 {
                data.extend(
                    row.iter()
                        .flat_map(|t| t.data[t.width*y .. t.width*(y+1)]
                        .into_iter())
                );
                if width == 0 {
                    width = data.len();
                } else {
                    assert_eq!(data.len() % width, 0);
                }
            }
        }

        Image { data, width: width }
    }

    fn find_monsters(&self) -> Vec<Coord> {
        let (w, h) = self.size();
        let mut monsters = Vec::new();
        for y in 0 .. h-3 {
            for x in 0 .. w-20 {
                if FRED.into_iter()
                    .all(|(dx, dy)| self.data[w*(y+dy) + x+dx] != 0)
                {
                    monsters.push((x, y));
                }
            }
        }

        monsters
    }

    fn count_roughness(&self) -> u32 {
        self.data.iter()
            .map(|&d| d as u32)
            .sum()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        let (w, h) = self.size();
        println!("{}x{}", w, h);

        for y in 0..h {
            for x in 0..w {
                print!("{}", if self.data[w*y + x] == 0 { '.' } else { '#' });
            }
            println!();
        }
    }
}

impl<'a> Sub for &'a Image {
    type Output = Image;

    fn sub(self, rhs: &'a Image) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        Image {
            width: self.width,
            data: self.data.iter()
                .zip(rhs.data.iter())
                .map(|(a, b)| a & (b^1))
                .collect()
        }
    }
}


trait Pack<'a>: Iterator<Item=&'a u8> {
    fn pack(self) -> u16;
}

impl<'a, I> Pack<'a> for I
where
    I: Iterator<Item=&'a u8>
{
    fn pack(self) -> u16 {
        self.fold(0, |a, &v| a<<1 | v as u16)
    }
}


//----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn check1(input: &str, size: usize, exp_corners: &[u32]) {
        let tiles = read(&mut input.as_bytes());
        assert_eq!(tiles.len(), size*size);
        for t in tiles.values() {
            assert_eq!(t.size(), (10, 10));
        }

        let match_ = cache_matches(&tiles);
        let corners = find_corners(&tiles, &match_);
        assert_eq!(match_.len(), 4*size*(size-1));

        assert_eq!(
            corners.iter().collect::<HashSet<_>>(),
            exp_corners.iter().collect::<HashSet<_>>(),
        );
    }

    #[test]
    fn ex0_corners() {
        check1(EX0, 3, &[ 1951, 3079, 2971, 1171 ]);
    }

    #[test]
    fn answer1() {
        check1(INPUT, 12, &[ 3557, 3769, 1019, 1097 ]);
    }


    fn check2(input: &str, exp_size: usize, nmonsters: usize, roughness: u32)
        -> Vec<Coord>
    {
        let mut tiles = read(&mut input.as_bytes());
        let match_ = cache_matches(&tiles);
        let corners = find_corners(&tiles, &match_);

        let (mosaic, size) = assemble_tiles(&match_, &corners);
        remove_borders(&mut tiles);
        let img = Image::from_mosaic(&tiles, &mosaic, size);
        assert_eq!(img.size(), (exp_size, exp_size));

        let (oriented, monsters) = hunt_monsters(&img);
        assert_eq!(oriented.size(), (exp_size, exp_size));
        assert_eq!(monsters.len(), nmonsters);

        let hab = &oriented - &mask_monsters(&monsters, img.size());
        assert_eq!(hab.count_roughness(), roughness);
        monsters
    }

    #[test]
    fn ex0_roughness() {
        check2(EX0, 24, 2, 273);
    }

    #[test]
    fn answer2() {
        check2(INPUT, 96, 23, 2161);
    }

    pub const EX0: &str = include_str!("../../ex0.txt");
    pub const INPUT: &str = include_str!("../../input.txt");
}
