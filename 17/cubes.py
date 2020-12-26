#!/usr/bin/env python3
from operator import or_
from functools import reduce
from itertools import product
from collections import Counter

B = 32			# bits / component -1 for separation
M = (1 << B-1) - 1	# component mask


def main():
    from sys import argv
    with open(argv[1]) as file:
        seed = read(file)
    #print(len(seed), seed)

    print('part[1]:', len(sim(seed, 3)))
    print('part[2]:', len(sim(seed, 4)))


def sim(state, dims, n=6):
    dirs = cache_dirs(dims)
    mask = reduce(or_, (M << B*i for i in range(dims)))

    for i in range(n):
        state = step(state, dirs, mask)
        #print(f'[{i}]', len(state))

    return state


def step(prev, dirs, mask):
    next, hood = set(), Counter()

    for p in prev:
        n = 0
        for dp in dirs:
            q = p+dp & mask
            if q in prev:
                n += 1
            else:
                hood[q] += 1

        if 2 <= n <= 3:
            next.add(p)

    next.update(p for p, n in hood.items() if n == 3)

    return next


def cache_dirs(dims):
    axes = (
        ((-1 & M) << B*i, 0, 1 << B*i)
        for i in range(dims)
    )

    return tuple(
        reduce(or_, dp)
        for dp in product(*axes)
        if any(u for u in dp)
    )


def read(file):
    return {
        y<<B | x
        for y, line in enumerate(file)
        for x, c in enumerate(line)
        if c == '#'
    }


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test_dirs_3d():
    assert len(cache_dirs(3)) == 26

def test_ex0_3d():
    assert len(sim(read(open('ex0.txt')), 3)) == 112

def test_answer1():
    assert len(sim(read(open('input.txt')), 3)) == 202


#------------------------------------------------------------------------------
# part 2 examples

def test_dirs_4d():
    assert len(cache_dirs(4)) == 80

def test_ex0_4d():
    assert len(sim(read(open('ex0.txt')), 4)) == 848

def test_answer1():
    assert len(sim(read(open('input.txt')), 4)) == 2028
