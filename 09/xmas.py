#!/usr/bin/env python3
from array import array
from itertools import chain, accumulate

def main():
    from sys import argv
    with open(argv[1]) as file:
        data = read(file)
    #print(min(data), max(data))

    # override default preamble/window size for smaller example
    # NB use 5 for ex0
    n = int(argv[2]) if len(argv) > 2 else 25

    x = find_first_invalid(data, n)
    print('part[1]:', x)

    i, j = find_range_totaling(data, x)
    print('part[2]:', calc_weakness(data[i:j]))


def read(file):
    return memoryview(array('q', (int(line) for line in file)))


def find_first_invalid(data, n=25):
    # sliding window lookup for faster sum checks
    window = { x: i for i, x in enumerate(data[:n]) }

    for i, x in enumerate(data[n:], n):
        if not any(window.get(x-y, 0) > j
                   for j, y in enumerate(data[i-n:i], i-n)):
            return x

        # maintain sliding window
        x0 = data[i-n]
        if window[x0] == i-n:
            del window[x0]
        window[x] = i


def find_range_totaling(data, tgt):
    # subtract iterator => start of integration window
    subit = enumerate(data)
    acc, i = 0, -1

    # add iteration => end of integration window
    for j, x in enumerate(data):
        assert acc < tgt
        acc += x			# open window

        if acc > tgt:
            for i, y in subit:
                acc -= y		# close window
                if acc <= tgt:
                    break

        if acc == tgt and j >= i+2:
            return i+1, j+1


def calc_weakness(data):
    return min(data) + max(data)


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_cases0():
    data = array('q', range(1, 25+1))

    data.append(26)
    assert find_first_invalid(data) is None

    data[-1] = 49
    assert find_first_invalid(data) is None

    data[-1] = 100
    assert find_first_invalid(data) == 100

    data[-1] = 50
    assert find_first_invalid(data) == 50

def test1_cases1():
    data = array('q', chain(
        range(1, 19+1), range(21, 25+1), (45,)
    ))

    data.append(26)
    assert find_first_invalid(data) is None

    data[-1] = 65
    assert find_first_invalid(data) == 65

    data[-1] = 64
    assert find_first_invalid(data) is None

    data[-1] = 66
    assert find_first_invalid(data) is None

def test1_ex0():
    data = read(open('ex0.txt'))
    assert find_first_invalid(data, 5) == 127

def test1_answer():
    data = read(open('input.txt'))
    assert find_first_invalid(data) == 144381670


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    data = read(open('ex0.txt'))
    i, j = find_range_totaling(data, 127)
    assert (i, j) == (2, 6)
    assert sum(data[i:j]) == 127
    assert calc_weakness(data[i:j]) == 62

def test2_answer():
    data = read(open('input.txt'))
    i, j = find_range_totaling(data, 144381670)
    assert (i, j) == (451, 468)
    assert calc_weakness(data[i:j]) == 20532569
