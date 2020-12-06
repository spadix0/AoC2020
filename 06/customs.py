#!/usr/bin/env python3
from functools import reduce
zet = frozenset

def main():
    from sys import argv
    with open(argv[1]) as file:
        groups = parse_groups(file.read())
    #print(groups)

    print('part[1]:', part1(groups))
    print('part[2]:', part2(groups))


def part1(groups):
    return sum(map(count_anyone_yes, groups))


def part2(groups):
    return sum(map(count_everyone_yes, groups))


def count_anyone_yes(group):
    return len(reduce(zet.union, group))


def count_everyone_yes(group):
    return len(reduce(zet.intersection, group))


def parse_groups(slurpee):
    return [
        [ zet(entry) for entry in group.split('\n') if entry ]
        for group in slurpee.split('\n\n')
    ]


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    groups = parse_groups(open('ex0.txt').read())
    assert len(groups) == 5
    assert [ 3, 3, 3, 1, 1 ] == [
        count_anyone_yes(g) for g in groups
    ]
    assert part1(groups) == 11

def test1_answer():
    groups = parse_groups(open('input.txt').read())
    assert part1(groups) == 6590


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    groups = parse_groups(open('ex0.txt').read())
    assert len(groups) == 5
    assert [ 3, 0, 1, 1, 1 ] == [
        count_everyone_yes(g) for g in groups
    ]
    assert part2(groups) == 6

def test2_answer():
    groups = parse_groups(open('input.txt').read())
    assert part2(groups) == 3288
