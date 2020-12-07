#!/usr/bin/env python3
from functools import lru_cache
import re

MY_BAG = 'shiny gold'


def main():
    from sys import argv
    with open(argv[1]) as file:
        rules = read(file)
    #print(rules)
    #print(reachable_from(rules))

    print('part[1]:', len(reachable_from(rules)))
    print('part[2]:', count_contents(rules))


def reachable_from(graph, dst=MY_BAG):
    vis = { dst: True }

    def can_reach(node):
        if (reached := vis.get(node)) is None:
            reached = vis[node] = any(can_reach(edge) for edge in graph[node])
        return reached

    for node in graph:
        can_reach(node)

    return {
        node for node, reached in vis.items()
        if reached and node is not dst
    }


def count_contents(graph, src=MY_BAG):
    @lru_cache(maxsize=None)
    def count_rec(node):
        return 1 + sum(weight * count_rec(edge)
                       for edge, weight in graph[node].items())

    return count_rec(src) - 1	# don't include src


def read(file):
    return {
        rule[0]: rule[1]
        for line in file
        if (rule := parse_rule(line))
    }


RULE_RE = re.compile(r'^([\w\s]+?) bags contain (.*).\s*$')
CONTENT_RE = re.compile(r'^(\d+) ([\w\s]+?) bags?$')

def parse_rule(rule):
    lhs, rhs = RULE_RE.match(rule).groups()

    if rhs == 'no other bags':
        return lhs, {}

    return lhs, {
        m[1]: int(m[0])
        for sub in rhs.split(', ')
        if (m := CONTENT_RE.match(sub).groups())
    }


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test_ex0_read():
    rules = read(open('ex0.txt'))
    assert len(rules) == 9
    assert sum(rules['faded blue'].values()) == 0
    assert sum(rules['vibrant plum'].values()) == 11

def test1_ex0():
    bags = reachable_from(read(open('ex0.txt')))
    assert bags == {
        'bright white', 'muted yellow', 'dark orange', 'light red'
    }

def test1_answer():
    assert len(reachable_from(read(open('input.txt')))) == 316


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    rules = read(open('ex0.txt'))
    cases = [
        ('faded blue', 0),
        ('dotted black', 0),
        ('dark olive', 7),
        ('vibrant plum', 11),
        ('shiny gold', 32),
    ]
    for bag, exp in cases:
        assert count_contents(rules, bag) == exp

def test2_ex1():
    assert count_contents(read(open('ex1.txt'))) == 126

def test2_answer():
    assert count_contents(read(open('input.txt'))) == 11310
