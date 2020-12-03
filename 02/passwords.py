#!/usr/bin/env python3

def main():
    from sys import argv
    with open(argv[1]) as file:
        data = read(file)

    print('part[1]:', count_valid(data, validate1))
    print('part[2]:', count_valid(data, validate2))


def parse(s):
    s, passwd = s.strip().split(': ')
    s, char = s.split()
    lo, hi = s.split('-')
    return int(lo), int(hi), char, passwd


def read(file):
    return [ parse(line) for line in file ]


def validate1(lo, hi, char, passwd):
    return lo <= sum(1 for c in passwd if c == char) <= hi


def validate2(lo, hi, char, passwd):
    return (passwd[lo-1] == char) != (passwd[hi-1] == char)


def count_valid(data, validate):
    return sum(1 for ent in data if validate(*ent))


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

import pytest

@pytest.fixture
def ex0_db():
    from io import StringIO
    return read(StringIO(EX0))


def test1_ex0_parse(ex0_db):
    assert ex0_db == [
        (1, 3, 'a', 'abcde'),
        (1, 3, 'b', 'cdefg'),
        (2, 9, 'c', 'ccccccccc'),
    ]

def test1_ex0_validate(ex0_db):
    expects = (1, 0, 1)
    for ent, exp in zip(ex0_db, expects):
        act = validate1(*ent)
        assert act == bool(exp)

def test_ex0_count(ex0_db):
    assert count_valid(ex0_db, validate1) == 2


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0_validate(ex0_db):
    expects = (1, 0, 0)
    for ent, exp in zip(ex0_db, expects):
        assert validate2(*ent) == bool(exp)

def test2_ex0_count(ex0_db):
    assert count_valid(ex0_db, validate2) == 1


EX0 = '''\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
'''
