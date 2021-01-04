#!/usr/bin/env python3
from functools import reduce
from collections import Counter
import re

B = 32			# bits / component -1 for separation
M = (1 << B-1) - 1	# component mask

DIRS = {
    'e': 1 | M<<B,
    'w': M | 1<<B,

    'ne': 1,
    'nw': 1<<B,

    'sw': M,
    'se': M<<B,
}
MASK = M | M<<B


def main():
    from sys import argv
    with open(argv[1]) as file:
        paths = read(file)
    #print(paths)

    floor = toggle(paths)
    print('part[1]:', len(floor))
    print('part[2]:', len(run(floor, 100)))


def toggle(paths):
    dsts = Counter(
        reduce(lambda p, dp: p+dp & MASK, seq)
        for seq in paths
    )
    return set(p for p, v in dsts.items() if v & 1)


def run(floor, n):
    for _ in range(n):
        floor = step(floor)
    return floor


def step(prev):
    next, hood = set(), Counter()
    for p in prev:
        n = 0
        for dp in DIRS.values():
            q = p+dp & MASK
            if q in prev:
                n += 1
            else:
                hood[q] += 1

        if 1 <= n <= 2:
            next.add(p)

    next.update(p for p, n in hood.items() if n == 2)

    return next


def read(file):
    DIR_RE = re.compile(r'[ns]?[ew]')
    return [ [DIRS[d[0]] for d in DIR_RE.finditer(line)] for line in file ]


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    paths = read(open('ex0.txt'))
    assert len(paths) == 20
    assert len(toggle(paths)) == 10

def test1_answer():
    assert len(toggle(read(open('input.txt')))) == 263


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    assert len(run(toggle(read(open('ex0.txt'))), 100)) == 2208

def test2_answer():
    assert len(run(toggle(read(open('input.txt'))), 100)) == 3649
