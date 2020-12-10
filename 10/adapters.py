#!/usr/bin/env python3
from operator import mul
from itertools import groupby
from functools import cache, reduce	# NB new in 3.9
from collections import Counter

def main():
    from sys import argv
    with open(argv[1]) as file:
        diff = read_diffs(file)
    #print(''.join(str(d) for d in diff))

    print('part[1]:', mul(*histogram(diff)))
    print('part[2]:', count_combos(diff))


def histogram(diff):
    hist = Counter(diff)
    #print(hist)
    assert hist.keys() == { 1, 3 }
    return hist[1], hist[3]


def count_combos(diff):
    # 3-diffs "pin" their endpoints => those must both be included
    # just find runs of 1-diffs and multiply those independent combinations
    return reduce(mul, (
        sum3_combos(sum(g))
        for d, g in groupby(diff)
        if d == 1
    ))


@cache
def sum3_combos(n):
    if n < 0:
        return 0
    if n == 0:
        return 1
    return sum3_combos(n-3) + sum3_combos(n-2) + sum3_combos(n-1)


def read_diffs(file):
    data = [ int(line) for line in file ]
    data.append(0)
    data.sort()
    diff = [ b - a for a, b in zip(data, data[1:]) ]
    diff.append(3)
    return diff


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    assert histogram(read_diffs(open('ex0.txt'))) == (7, 5)

def test1_ex1():
    assert histogram(read_diffs(open('ex1.txt'))) == (22, 10)

def test1_answer():
    assert histogram(read_diffs(open('input.txt'))) == (70, 27)


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    assert count_combos(read_diffs(open('ex0.txt'))) == 8

def test2_ex1():
    assert count_combos(read_diffs(open('ex1.txt'))) == 19208

def test2_answer():
    assert count_combos(read_diffs(open('input.txt'))) == 49607173328384

def test_combos():
    for n, c in [
        (1, 1),
        (2, 2),
        (3, 4),
        (4, 7),
    ]:
        assert sum3_combos(n) == c
