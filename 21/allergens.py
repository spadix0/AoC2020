#!/usr/bin/env python3
from functools import reduce
from operator import or_
zet = frozenset

def main():
    from sys import argv
    with open(argv[1]) as file:
        data = read(file)
    #print(data)

    cons = constrain(data)
    #print('constraints:', cons)

    contam = union_all(cons.values())
    print(f'ingredients with allergens:\n    {contam}')

    print('part[1]:', count_except(data, contam))
    print('part[2]:', ','.join(canonize(match(cons))))


def constrain(data):
    cons = { }
    for lhs, rhs in data:
        for a in rhs:
            if a in cons:
                cons[a] &= lhs
            else:
                cons[a] = set(lhs)

    return cons


def union_all(sets):
    return reduce(or_, sets, set())


def count_except(data, filter):
    return sum(
        1 for lhs, _ in data
        for x in lhs
        if x not in filter
    )


def match(cons):
    unvis = list(cons)
    unvis.sort(key=lambda x: -len(cons[x]))

    dig = { }
    while unvis:
        for i in reversed(range(len(unvis))):
            if len(cons[(x := unvis[i])]) == 1:
                del unvis[i]
                break

        p = next(iter(cons[x]))
        print(f'    {x} in {p}')
        dig[x] = p
        for x in unvis:
            cons[x].discard(p)

    return dig


def canonize(dig):
    return [ dig[k] for k in sorted(dig) ]


def read(file):
    return [ parse1(line.strip()) for line in file ]


def parse1(s):
    lhs, rhs = s.rstrip(')').split(' (contains ')
    return zet(lhs.split()), zet(rhs.split(', '))


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    data = read(open('ex0.txt'))
    contam = union_all(constrain(data).values())
    assert contam == set('mxmxvkd sqjhc fvjkl'.split())
    assert count_except(data, contam) == 5

def test1_answer():
    data = read(open('input.txt'))
    assert count_except(data, union_all(constrain(data).values())) == 2786


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    dig = match(constrain(read(open('ex0.txt'))))
    assert dig == {
        'dairy': 'mxmxvkd',
        'fish': 'sqjhc',
        'soy': 'fvjkl',
    }
    cdil = canonize(dig)
    assert cdil == 'mxmxvkd,sqjhc,fvjkl'.split(',')

def test2_answer():
    cdil = canonize(match(constrain(read(open('input.txt')))))
    assert ','.join(cdil) == 'prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq'
