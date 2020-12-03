#!/usr/bin/env python3

def main():
    from sys import argv, stdin
    with open(argv[1]) as f:
        data = read(f)

    a, b = brutish2(data)
    print(f'part[1]: {a} * {b} = {a*b}')
    assert (a, b) == linear2(data)

    a, b, c = brutish3(data)
    print(f'part[2]: {a} * {b} * {c} = {a*b*c}')
    assert (a, b, c) == quad3(data)


def read(file):
    return [ int(line) for line in file ]


# initial, brute force hacks
# quadratic (cubic) time, but no extra storage

def brutish2(data, target=2020):
    for i,a in enumerate(data):
        for b in data[i+1:]:
            if a + b == target:
                return a, b


def brutish3(data, target=2020):
    for i,a in enumerate(data):
        for j,b in enumerate(data[i+1:], i+1):
            for c in data[j+1:]:
                if a + b + c == target:
                    return a, b, c


# polynomial degree reduction optimization
# linear (quadratic) time, but uses linear storage

# NB first one is much *slower* than brutish2 for small example input!
def linear2(data, target=2020):
    lut = { x: i for i,x in enumerate(data) }
    for i, a in enumerate(data):
        b = target - a
        if lut.get(b, -1) > i:
            return a, b


def quad3(data, target=2020):
    lut = { x: i for i, x in enumerate(data) }
    for i, a in enumerate(data):
        for j, b in enumerate(data[i+1:], i+1):
            c = target - a - b
            if lut.get(c, -1) > j:
                return a, b, c


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

import pytest

@pytest.fixture
def ex0():
    from io import StringIO
    return read(StringIO(EX0))

@pytest.fixture
def input():
    with open('input.txt') as file:
        return read(file)

@pytest.fixture(params=[brutish2, linear2])
def sum2_impl(request):
    return request.param


def test_ex0_parse(ex0):
    assert ex0 == [ 1721, 979, 366, 299, 675, 1456 ]

def test1_ex0_brutish(ex0):
    assert brutish2(ex0) == EX0_SEL2

def test1_ex0_linear(ex0):
    assert linear2(ex0) == EX0_SEL2

def test1_half_nodup(sum2_impl):
    # same element should not be considered multiple times
    assert sum2_impl([ 1010, 1009, 1011 ]) == (1009, 1011)

def test1_dup2(sum2_impl):
    # identical elements should be considered for each instance
    assert sum2_impl([ 1, 1010, 2, 1010, 3 ]) == (1010, 1010)

@pytest.mark.benchmark(group='part 1')
def testperf1_input_brutish(input, benchmark):
    benchmark(brutish2, input)

@pytest.mark.benchmark(group='part 1')
def testperf1_input_linear(input, benchmark):
    benchmark(linear2, input)


#------------------------------------------------------------------------------
# part 2 examples

@pytest.fixture(params=[brutish3, quad3])
def sum3_impl(request):
    return request.param

def test2_ex0_brutish(ex0):
    assert brutish3(ex0) == EX0_SEL3

def test2_ex0_quad(ex0):
    assert quad3(ex0) == EX0_SEL3

def test2_third_nodup(sum3_impl):
    # same element should not be considered multiple times
    assert sum3_impl([ 673, 1, 2, 672, 674, 3 ], 2019) == (673, 672, 674)

def test2_half_nodup(sum3_impl):
    assert sum3_impl([ 674, 673, *EX0_SEL3 ]) == EX0_SEL3

def test2_dup2(sum3_impl):
    # identical elements should be considered for each instance
    assert sum3_impl([ 674, 1, 673, 2, 673, 3 ]) == (674, 673, 673)

def test2_dup3(sum3_impl):
    assert sum3_impl([ 673, 1, 673, 2, 673, 3 ], 2019) == (673, 673, 673)

@pytest.mark.benchmark(group='part 2')
def testperf2_input_brutish(input, benchmark):
    benchmark(brutish3, input)

@pytest.mark.benchmark(group='part 2')
def testperf2_input_quad(input, benchmark):
    benchmark(quad3, input)


EX0 = '''\
1721
979
366
299
675
1456
'''

EX0_SEL2 = (1721, 299)
EX0_EXP1 = 514579

EX0_SEL3 = (979, 366, 675)
EX0_EXP2 = 241861950
