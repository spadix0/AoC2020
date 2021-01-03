#!/usr/bin/env python3
from itertools import count
from functools import reduce
from operator import mul, or_
from collections import namedtuple

FRED = [ # the sea monster
    0b00000000000000000010,
    0b10000110000110000111,
    0b01001001001001001000,
]


def main():
    from sys import argv
    with open(argv[1]) as file:
        tiles = read(file)
    #print(tiles)

    tiles, borders = extract_borders(tiles)
    match = cache_matches(borders)
    corners = find_corners(tiles, match)
    print('corners:', corners)
    print('part[1]:', reduce(mul, corners))

    mosaic = assemble_tiles(match, corners)
    img = Image.from_mosaic(tiles, mosaic)
    #img.dump()

    img, monsters = hunt_monsters(img)
    print(f'{len(monsters)} sea monsters')
    mash = mask_monsters(monsters, img.size)
    print('part[2]:', (img - mash).count_roughness())


def extract_borders(tiles):
    borders = {
        id: t.extract_border()
        for id, t in tiles.items()
    }
    tiles = {
        id: t.trim_border()
        for id, t in tiles.items()
    }
    return tiles, borders


def cache_matches(borders):
    # NB assume all matched edges are unique and outside edges are unmatched
    match, pattern = { }, { }

    for id, b in borders.items():
        for d, p in enumerate(b):
            if ref := pattern.get(p):
                match[id, d] = ref
                match[ref.id, ref.dir] = TileRef(id, d, ref[2])
                #print(f'{p:010b}:', (id, d), '->', ref[:2], ref[2])
            else:
                pattern[p] = TileRef(id, d, 1)
                pattern[flipped(p, 10)] = TileRef(id, d, 0)

    return match


def find_corners(tiles, match):
    # corners have exactly 2 matched edges
    return [
        id for id in tiles
        if sum(int((id, d) in match) for d in range(4)) == 2
    ]


def assemble_tiles(match, corners):
    # select arbitrary corner and orient (starting at TL)
    tlc = corners[0]
    mt = int((tlc, 3) in match)
    ml = int((tlc, 2) in match)
    rot = ml<<1 | ml ^ mt

    grid = [ [ TileRef(tlc, rot, 0) ] ]

    rem = len(match) // 2
    while rem > 0:
        rem -= 1

        # try to extend right
        prev = grid[-1][-1]
        if m := match.get(prev.match(0)):
            next = m.orient(0, prev.flip)
            if len(grid) > 1:
                # up must also match
                rem -= 1
                up = grid[-2][len(grid[-1])]
                assert next == match[up.match(1)].orient(1, up.flip)
            grid[-1].append(next)
        else:
            # or extend down to next row
            assert len(grid[-1]) == len(grid[0])
            prev = grid[-1][0]
            m = match[prev.match(1)]
            next = m.orient(1, prev.flip)
            grid.append([ next ])

    #for row in grid:
    #    print('   ', ' '.join(f'{t.id}({"+" if t.flip else "-"}{t.dir})'
    #                          for t in row))

    assert len(grid) == len(grid[0]), 'assume square'
    return grid


def hunt_monsters(img):
    for (img, _, _) in img.all_orientations():
        if freds := img.find_monsters():
            return img, freds

    assert not 'hunt fail'


def mask_monsters(monsters, size):
    mash = Image(0 for _ in range(size))
    rows = mash.rows
    for m in monsters:
        for i,r in enumerate(FRED):
            rows[m[1]+i] |= r << -m[0]

    return mash


#------------------------------------------------------------------------------
class TileRef(namedtuple('TileRef', 'id dir flip')):
    def match(self, side):
        return self.id, self.edge(side)

    # which edge of referenced tile to match given orientation and flip
    def edge(self, side):
        dir = self.dir
        if self.flip:
            side, dir = -side, -dir
        return side-dir & 3

    # new orientation for referenced tile to match adjacent tile
    def orient(self, side, flip):
        id, dir = self.id, self.dir
        flip ^= self.flip
        if flip and dir&1:
            dir = -dir
        return TileRef(id, 2-dir+side & 3, flip)


class Image:
    __slots__ = 'rows',

    def __init__(self, rows=()):
        self.rows = list(rows)

    @classmethod
    def from_mosaic(self, tiles, mosaic):
        img = Image()
        rows = img.rows
        for mrow in mosaic:
            trow = [
                tiles[ref.id].flipped(ref.flip).rotated(ref.dir)
                for ref in mrow
            ]
            tsz = trow[0].size
            for y in range(tsz):
                r = 0
                for t in trow:
                    r = r<<tsz | t.rows[y]
                rows.append(r)
        return img

    @property
    def size(self):
        # NB assuming square tiles/images
        return len(self.rows)

    def __repr__(self):
        w = self.size
        rows = '/'.join(f'{r:0{w}b}' for r in self.rows)
        return f'Image({rows})'

    def extract_border(self):
        rows, sz = self.rows, self.size
        # [right, bottom, left, top] rotated to right, unflipped
        return (
            flipped(column(rows, 0), sz),
            flipped(rows[-1], sz),
            column(rows, sz-1),
            rows[0],
        )

    def trim_border(self):
        m = (1 << self.size-2) - 1
        return Image(r>>1 & m for r in self.rows[1:-1])

    def flipped(self, flip=True):
        if flip:
            return Image(reversed(self.rows))
        else:
            return self

    def rotated(self, n=None):
        rows, s = self.rows, self.size
        if n == 0:
            return self
        elif n == 1:
            return Image(column(rows, i) for i in reversed(range(s)))
        elif n == 2:
            return Image(flipped(r, s) for r in reversed(rows))
        elif n == 3:
            return Image(flipped(column(rows, i), s) for i in range(s))
        else:
            assert None

    def all_orientations(self):
        img = self
        for i in range(4):
            if i > 0:
                img = img.rotated(1)
            yield img, i, 0

        img = self.flipped()
        for i in range(4):
            if i > 0:
                img = img.rotated(1)
            yield img, i, 1

    def find_monsters(self):
        rows = self.rows
        return [
            (-x, y)
            for y in range(0, len(rows)-len(FRED))
            for x in range(self.size-20)
            if all(
                rows[y+i]>>x & r == r
                for i, r in enumerate(FRED)
            )
        ]

    def count_roughness(self):
        return sum(bin(r).count('1') for r in self.rows)

    def __sub__(self, img):
        return Image(r & ~s for r, s in zip(self.rows, img.rows))

    def dump(self, id=None):
        TR = str.maketrans({'1':'#', '0':'.'})
        w = self.size
        if id is not None:
            print(f'[{id}]')
        for r in self.rows:
            print(f'{r:0{w}b}'.translate(TR))


def flipped(row, s):
    return reduce(or_, (
        (row>>i & 1) << s-1-i
        for i in range(s)
    ))


def column(rows, c):
    return reduce(or_, (
        (r>>c & 1) << i
        for i, r in enumerate(rows)
    ))


def read(file):
    TR_TILE = str.maketrans({'.':'0', '#':'1'})
    tiles, tile = { }, [ ]

    for line in file:
        line = line.strip()
        if line.startswith('Tile'):
            tile = tiles[int(line[4:].rstrip(':'))] = Image()
        elif line:
            tile.rows.append(int(line.translate(TR_TILE), 2))

    return tiles


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
def test_flipped():
    assert flipped(0b11110000, 8) == 0b00001111
    assert flipped(0b01011101, 8) == 0b10111010
    assert flipped(0b10100101, 8) == 0b10100101
    assert flipped(0b10100101, 10) == 0b1010010100
    assert flipped(0x96a57e, 24) == 0x7ea569

def test_ref_edge():
    # exhaust all cases
    cases = [
        #rot side flip => edge
        (0, 0, 0,  0),	(0, 1, 0,  1),	(0, 0, 1,  0),	(0, 1, 1,  3),
        (1, 0, 0,  3),	(1, 1, 0,  0),	(1, 0, 1,  1),	(1, 1, 1,  0),
        (2, 0, 0,  2),	(2, 1, 0,  3),	(2, 0, 1,  2),	(2, 1, 1,  1),
        (3, 0, 0,  1),	(3, 1, 0,  2),	(3, 0, 1,  3),	(3, 1, 1,  2),
    ]
    for rot, side, flip, edge in cases:
        assert TileRef(0, rot, flip).edge(side) == edge

def test_ref_orient():
    # exhaust all cases (uses flip aggregatation assumptions)
    cases = [
        #rot side flip => rot
        (0, 0, 0,  2),	(0, 1, 0,  3),	(0, 0, 1,  2),	(0, 1, 1,  3),
        (1, 0, 0,  1),	(1, 1, 0,  2),	(1, 0, 1,  3),	(1, 1, 1,  0),
        (2, 0, 0,  0),	(2, 1, 0,  1),	(2, 0, 1,  0),	(2, 1, 1,  1),
        (3, 0, 0,  3),	(3, 1, 0,  0),	(3, 0, 1,  1),	(3, 1, 1,  2),
    ]
    for rot, side, flip, exp in cases:
        exp = TileRef(42, exp, flip)
        assert TileRef(42, rot, 0).orient(side, flip) == exp
        assert TileRef(42, rot, flip).orient(side, 0) == exp
        assert TileRef(42, rot, 1).orient(side, 1-flip) == exp
        assert TileRef(42, rot, 1-flip).orient(side, 1) == exp


#------------------------------------------------------------------------------
# part 1 examples

def check1(path, size, corners):
    with open(path) as file:
        tiles = read(file)
    assert len(tiles) == size*size
    for t in tiles.values():
        assert t.size == 10

    trimmed, borders = extract_borders(tiles)
    assert len(trimmed) == size*size
    assert len(borders) == size*size
    for t in trimmed.values():
        assert t.size == 8

    match = cache_matches(borders)
    assert len(match) == 4*size*(size-1)

    assert set(find_corners(trimmed, match)) == set(corners)

def test1_ex0():
    check1('ex0.txt', 3, { 1951, 3079, 2971, 1171 })

def test1_answer():
    check1('input.txt', 12, { 3557, 3769, 1019, 1097 })


#------------------------------------------------------------------------------
# part 2 examples

def check2(path, size, nmonsters, roughness):
    with open(path) as file:
        tiles = read(file)

    trimmed, borders = extract_borders(tiles)
    match = cache_matches(borders)
    mosaic = assemble_tiles(match, find_corners(trimmed, match))

    img = Image.from_mosaic(trimmed, mosaic)
    assert img.size == size

    oriented, monsters = hunt_monsters(img)
    assert oriented.size == size
    assert len(monsters) == nmonsters

    habitat = oriented - mask_monsters(monsters, oriented.size)
    assert habitat.count_roughness() == roughness
    return monsters

def test2_ex0():
    check2('ex0.txt', 24, 2, 273)

def test2_answer():
    check2('input.txt', 96, 23, 2161)
