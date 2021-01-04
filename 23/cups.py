#!/usr/bin/env python3
from array import array

def main():
    from sys import argv
    with open(argv[1]) as file:
        seed = parse(file.read().strip())
    #print(seed)

    cups = init(seed, len(seed))
    #print(dump8(cups), cups)
    print('part[1]:', dump8(run(cups, 100)))

    cups = init(seed, 1_000_000)
    _, c2, c3 = collect(run(cups, 10_000_000), 1, 3)
    print(f'part[2]: {c2} * {c3} = {c2*c3}')


def parse(line):
    return [ int(c) for c in line ]

def init(seed, n):
    cups = array('I', (i+1 for i in range(n+1)))
    for c0, c1 in zip(seed, seed[1:]):
        cups[c0] = c1

    if n > len(seed):
        cups[seed[-1]] = len(seed) + 1
        cups[n] = seed[0]
    else:
        cups[seed[-1]] = seed[0]

    cups[0] = seed[0]
    return cups


def run(cups, n):
    c = cups[0]
    for _ in range(n):
        c = step(cups, c)
    cups[0] = c
    return cups


def step(cups, c):
    p0 = cups[c]
    p1 = cups[p0]
    p2 = cups[p1]
    cups[c] = cups[p2]
    d = c - 1
    while not d or d == p0 or d == p1 or d == p2:
        if not d:
            d += len(cups) - 1
        else:
            d -= 1
    cups[p2] = cups[d]
    cups[d] = p0
    return cups[c]


def collect(cups, c, n):
    seq = [ c ]
    while len(seq) < n:
        seq.append(cups[seq[-1]])
    return seq


def dump8(cups):
    return ''.join(str(s) for s in collect(cups, 1, 9))[1:]


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    cups = init(parse('389125467'), 9)
    assert list(cups) == [3, 2, 5, 8, 6, 4, 7, 3, 9, 1]

    run(cups, 1)
    assert collect(cups, cups[0], 9) == [2, 8, 9, 1, 5, 4, 6, 7, 3]

    run(cups, 9)
    assert collect(cups, cups[0], 9) == [8, 3, 7, 4, 1, 9, 2, 6, 5]

    assert dump8(run(cups, 90)) == '67384529'

def test1_answer():
    assert dump8(run(init(parse('364289715'), 9), 100)) == '98645732'


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    cups = init(parse('389125467'), 1_000_000)
    assert collect(run(cups, 10_000_000), 1, 3) == [1, 934001, 159792]

def test2_answer():
    cups = init(parse('364289715'), 1_000_000)
    assert collect(run(cups, 10_000_000), 1, 3) == [1, 929588, 741727]
