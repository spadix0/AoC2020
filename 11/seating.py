#!/usr/bin/env python3
from array import array
from itertools import chain, count

# 8 neighborhood
DIRS = (
    (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)
)


def main():
    from sys import argv
    with open(argv[1]) as file:
        seats, width = read(file)

    nodes, edges = cache_adjacent(seats, width)
    print('part[1]:', sum(run_until_stable(nodes, edges, 4)))

    nodes, edges = cache_visible(seats, width)
    print('part[2]:', sum(run_until_stable(nodes, edges, 5)))


def read(file):
    seats, w = bytearray(), None
    for line in file:
        # pad w/floor on all sides to simplify boundary conditions
        seats.append(0)
        seats.extend(int(c == 'L') for c in line.strip())
        seats.append(0)
        if not w:
            w = len(seats)
            seats[:0] = (0 for _ in range(w))	# top pad
    seats.extend(0 for _ in range(w))		# bottom pad

    return seats, w


# optimized DOD implementations that cache adjacency graph

def cache_adjacent(seats, w):
    inode, n = compress_seats(seats, w)
    nodes = bytearray(0 for _ in range(n))
    edges = array('i')

    for p, i in enumerate(inode):
        if i >= 0:
            n0 = len(edges)
            edges.extend(
                q for q in chain(
                    inode[p-w-1:p-w+1+1],
                    inode[p-1:p-1+1], inode[p+1:p+1+1],
                    inode[p+w-1:p+w+1+1],
                )
                if q >= 0
            )
            nodes[i] = len(edges) - n0

    return nodes, edges


def cache_visible(seats, w):
    h = len(seats) // w
    inode, n = compress_seats(seats, w)
    nodes = bytearray(0 for _ in range(n))
    edges = array('i')

    def search_line(x, y, dx, dy):
        while True:
            x += dx
            y += dy
            if not (0 <= x < w and 0 <= y < h):
                return
            if (j := inode[w*y + x]) >= 0:
                return j

    for p, i in enumerate(inode):
        if i >= 0:
            n0 = len(edges)
            edges.extend(
                j for dx, dy in DIRS
                if (j := search_line(p%w, p//w, dx, dy))
            )
            nodes[i] = len(edges) - n0

    return nodes, edges


def compress_seats(seats, w):
    # map grid to compressed seat indices
    init = iter(range(len(seats)+1))
    inode = array('i', (next(init) if c else -1 for c in seats))
    return inode, next(init)


def run_until_stable(nodes, *args):
    prev, next = None, bytearray(len(nodes))
    while next != prev:
        prev, next = next, step(next, nodes, *args)
    return next


def step(prev, nodes, edges, thresh):
    acc = bytearray(len(nodes))
    i = 0
    for n, x in zip(nodes, prev):
        if x:
            for j in range(i, i+n):
                acc[edges[j]] += 1
        i += n
    assert i == len(edges)

    return bytearray(
        n < thresh if x else not n
        for x, n in zip(prev, acc)
    )


# initial reference implementations w/simple for loops for comparison

def run_adjacent(seats, w):
    prev, next = None, bytearray(len(seats))
    dirs = tuple(w*dy + dx for dx,dy in DIRS)

    while next != prev:
        prev, next = next, bytearray(len(seats))

        for i, s, p in zip(count(), seats, prev):
            if s:
                n = sum(prev[i+di] for di in dirs)
                next[i] = n < 4 if p else not n

    return next


def run_visible(seats, w):
    h = len(seats) // w
    prev, next = None, bytearray(len(seats))

    while next != prev:
        prev, next = next, bytearray(len(seats))

        for y0 in range(h):
            for x0 in range(w):
                i0 = w*y0 + x0
                if seats[i0]:
                    n = 0
                    for dx, dy in DIRS:
                        x, y = x0+dx, y0+dy
                        while 0 <= x < w and 0 <= y < h:
                            i = w*y + x
                            if seats[i]:
                                n += prev[i]
                                break
                            x, y = x+dx, y+dy
                    next[i0] = n < 5 if prev[i0] else not n

    return next


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

import pytest

@pytest.fixture
def ex0():
    with open('ex0.txt') as file:
        return read(file)

@pytest.fixture
def input():
    with open('input.txt') as file:
        return read(file)


def test1_ex0_graph(ex0):
    graph = cache_adjacent(*ex0)
    assert sum(run_until_stable(*graph, 4)) == 37

def test1_ex0_grid(ex0):
    assert sum(run_adjacent(*ex0)) == 37

def test1_input_graph(input):
    graph = cache_adjacent(*input)
    assert sum(run_until_stable(*graph, 4)) == 2368

def test1_input_grid(input):
    assert sum(run_adjacent(*input)) == 2368


@pytest.mark.benchmark(group='adjacent small')
def testperf1_ex0_graph(ex0, benchmark):
    graph = cache_adjacent(*ex0)
    benchmark(run_until_stable, *graph, 4)

@pytest.mark.benchmark(group='adjacent small')
def testperf1_ex0_grid(ex0, benchmark):
    benchmark(run_adjacent, *ex0)

@pytest.mark.benchmark(group='adjacent nominal')
def testperf1_graph(input, benchmark):
    graph = cache_adjacent(*input)
    benchmark(run_until_stable, *graph, 4)

@pytest.mark.benchmark(group='adjacent nominal')
def testperf1_grid(input, benchmark):
    benchmark(run_adjacent, *input)


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0_graph(ex0):
    graph = cache_visible(*ex0)
    assert sum(run_until_stable(*graph, 5)) == 26

def test2_ex0_grid(ex0):
    assert sum(run_visible(*ex0)) == 26

def test2_input_graph(input):
    graph = cache_visible(*input)
    assert sum(run_until_stable(*graph, 5)) == 2124

def test2_input_grid(input):
    assert sum(run_visible(*input)) == 2124


@pytest.mark.benchmark(group='visible small')
def testperf2_ex0_graph(ex0, benchmark):
    graph = cache_visible(*ex0)
    benchmark(run_until_stable, *graph, 5)

@pytest.mark.benchmark(group='visible small')
def testperf2_ex0_grid(ex0, benchmark):
    benchmark(run_visible, *ex0)

@pytest.mark.benchmark(group='visible nominal')
def testperf2_graph(input, benchmark):
    graph = cache_visible(*input)
    benchmark(run_until_stable, *graph, 5)

@pytest.mark.benchmark(group='visible nominal')
def testperf2_grid(input, benchmark):
    benchmark(run_visible, *input)
