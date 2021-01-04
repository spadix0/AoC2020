#!/usr/bin/env python3
from itertools import islice, takewhile
from collections import deque

def main():
    from sys import argv
    with open(argv[1]) as file:
        init = read(file)
    #print(init)

    decks = [ deque(d) for d in init ]
    print('part[1]:', score(decks[play(decks)]))

    decks = [ deque(d) for d in init ]
    print('part[2]:', score(decks[play_rec(*decks)]))


def play(decks):
    while (win := round(*decks)) is None:
        pass
    return win


def round(d0, d1):
    c0, c1 = d0.popleft(), d1.popleft()
    if c0 > c1:
        d0.extend((c0, c1))
        if not d1: return 0
    else:
        d1.extend((c1, c0))
        if not d0: return 1


def play_rec(d0, d1):
    memo = set()
    while True:
        if (h := (bytes(d0), bytes(d1))) in memo:
            return 0
        memo.add(h)

        c0, c1 = d0.popleft(), d1.popleft()
        if len(d0) >= c0 and len(d1) >= c1:
            win = play_rec(deque(islice(d0, 0, c0)), deque(islice(d1, 0, c1)))
        else:
            win = int(c1 > c0)

        if win == 0:
            d0.extend((c0, c1))
            if not d1: return 0
        else:
            d1.extend((c1, c0))
            if not d0: return 1


def score(deck):
    return sum(i*c for i, c in enumerate(reversed(deck), 1))


def read(file):
    def read_decks(file):
        for line in file:
            assert line.startswith('Player ')
            yield [
                int(c) for c in takewhile(
                    bool, (line.strip() for line in file))
            ]

    return list(read_decks(file))


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    decks = [ deque(d) for d in read(open('ex0.txt')) ]
    assert play(decks) == 1
    assert list(decks[1]) == [3, 2, 10, 6, 8, 5, 9, 4, 7, 1]
    assert score(decks[1]) == 306

def test1_answer():
    decks = [ deque(d) for d in read(open('input.txt')) ]
    assert play(decks) == 1
    assert score(decks[1]) == 34127


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    decks = [ deque(d) for d in read(open('ex0.txt')) ]
    assert play_rec(*decks) == 1
    assert list(decks[1]) == [7, 5, 6, 2, 4, 1, 10, 8, 9, 3]
    assert score(decks[1]) == 291

def test2_answer():
    decks = [ deque(d) for d in read(open('input.txt')) ]
    assert play_rec(*decks) == 1
    assert score(decks[1]) == 32054
