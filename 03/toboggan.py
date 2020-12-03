#!/usr/bin/env python3
from functools import reduce
from operator import mul

ALL_SLOPES = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
]

def main():
    from sys import argv
    with open(argv[1]) as file:
        forest = Forest(file.readlines())

    #print(forest.dumped())
    print('part[1]:', forest.count_trees(3))

    paths = all_paths(forest)
    print('part[2]:', paths, reduce(mul, paths))


def all_paths(forest):
    return [
        forest.count_trees(dx, dy)
        for dx, dy in ALL_SLOPES
    ]


class Forest:
    def __init__(self, pattern):
        self.width = len(pattern[0].strip())
        self.grid = [ _parse_row(row.strip()) for row in pattern ]

    def dumped(self):
        w = self.width
        remap = str.maketrans('01', '.#')
        return '\n'.join(
            ''.join(reversed(f'{row:0{w}b}'.translate(remap)))
            for row in self.grid)

    def count_trees(self, dx, dy=1):
        w = self.width
        return sum(
            row >> dx*y_dy%w & 1
            for y_dy, row in enumerate(self.grid[::dy]))


def _parse_row(s):
    return sum(1<<i for i,c in enumerate(s) if c == '#')


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def ex0():
    return Forest(EX0.split())


def test_ex0_parse():
    assert ex0().dumped() == EX0.strip()

def test1_ex0():
    assert ex0().count_trees(3) == 7


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    assert all_paths(ex0()) == [ 2, 7, 3, 4, 2 ]


EX0 = '''\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
'''
