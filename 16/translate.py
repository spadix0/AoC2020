#!/usr/bin/env python3
from itertools import chain
from functools import reduce
from operator import mul, or_
from collections import defaultdict, deque
from bisect import bisect_left, bisect_right

chainit = chain.from_iterable

def main():
    from sys import argv
    with open(argv[1]) as file:
        fields, own, nearby = parse(file.read())
    #print(fields)
    #print(own)
    #print(nearby)

    ranges, names = build_lookup(fields)

    reject = find_invalid(ranges, names, chainit(nearby))
    print('part[1]:', sum(reject))

    cons = constrain_valid(ranges, names, own, nearby)
    names = resolve_fields(cons)
    print('part[2]:', hash_departure(names, own))
    print('field order:', ', '.join(names))


def find_invalid(ranges, names, values):
    return [
        v for v in values
        if not names[bisect_right(ranges, v) - 1]
    ]


def hash_departure(names, own):
    departs = [
        v for nm, v in zip(names, own)
        if nm.startswith('departure')
    ]
    return reduce(mul, departs or (0,))


def resolve_fields(cons):
    # collect reverse edges of bipartite graph
    # and frontier queue of nodes to resolve
    front, rev = deque(), defaultdict(set)
    for src, fwd in enumerate(cons):
        if len(fwd) == 1:
            front.append(src)
        for dst in fwd:
            rev[dst].add(src)

    match = [ None for _ in cons ]

    # match frontier src to single dst and follow reverse edges
    # to remove all other forward edges that refer to dst
    while front:
        src = front.popleft()
        fwd = cons[src]
        assert len(fwd) == 1
        assert match[src] is None

        dst = match[src] = fwd.pop()

        for s in rev[dst]:
            f = cons[s]
            f.discard(dst)
            if len(f) == 1:
                front.append(s)

    assert all(m is not None for m in match)
    return match


def constrain_valid(ranges, names, own, nearby):
    cons = [ names[bisect_right(ranges, v) - 1].copy() for v in own ]

    for t in nearby:
        tc = [ names[bisect_right(ranges, v) - 1] for v in t ]
        if all(tc):
            for acc, n in zip(cons, tc):
                acc &= n

    return cons


# generate table of value intervals suitable for binary search
def build_lookup(fields):
    ranges = sorted({
        v
        for nm, rngs in fields.items()
        for r in rngs
        for v in (r[0], r[1]+1)
    })
    names = [ set() for _ in ranges ]

    for nm, rngs in fields.items():
        for lo, hi in rngs:
            i0 = bisect_left(ranges, lo)
            i1 = bisect_left(ranges, hi+1)
            for i in range(i0, i1):
                names[i].add(nm)

    return ranges, names


def parse(s):
    grps = s.strip().split('\n\n')

    fields = {
        fv[0]: fv[1] for line in grps[0].split('\n')
        if (fv := parse_field(line))
    }

    lines = iter(grps[1].split('\n'))
    assert next(lines) == 'your ticket:'
    own = parse_ticket(next(lines))

    lines = iter(grps[2].split('\n'))
    assert next(lines) == 'nearby tickets:'
    nearby = [ parse_ticket(line) for line in lines ]

    return fields, own, nearby


def parse_field(s):
    name, ranges = s.split(': ')
    return name, [
        (int(v[0]), int(v[1]))
        for r in ranges.split(' or ')
        if (v := r.split('-'))
    ]


def parse_ticket(s):
    return [ int(x) for x in s.split(',') ]


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

import pytest

def read_and_build(path):
    with open(path) as file:
        fields, own, nearby = parse(file.read())
        return *build_lookup(fields), own, nearby


def test1_ex0():
    ranges, names, own, nearby = read_and_build('ex0.txt')
    assert reduce(or_, names, set()) == { 'class', 'row', 'seat' }
    assert own == [7, 1, 14]
    reject = find_invalid(ranges, names, chainit(nearby))
    assert set(reject) == { 4, 55, 12 }

def test1_ex1():
    ranges, names, own, nearby = read_and_build('ex1.txt')
    assert reduce(or_, names, set()) == { 'class', 'row', 'seat' }
    assert own == [11, 12, 13]
    assert not find_invalid(ranges, names, chainit(nearby))

def test1_answer():
    ranges, names, _, nearby = read_and_build('input.txt')
    assert sum(find_invalid(ranges, names, chainit(nearby))) == 29759


#------------------------------------------------------------------------------
# part 2 examples

def read_and_resolve(path):
    ranges, names, own, nearby = read_and_build(path)
    cons = constrain_valid(ranges, names, own, nearby)
    return resolve_fields(cons), own


def test2_ex0():
    names, _ = read_and_resolve('ex0.txt')
    assert names == 'row class seat'.split()

def test2_ex1():
    names, _ = read_and_resolve('ex1.txt')
    assert names == 'row class seat'.split()

def test2_answer():
    assert hash_departure(*read_and_resolve('input.txt'))
