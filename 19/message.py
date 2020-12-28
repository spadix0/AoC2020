#!/usr/bin/env python3
from functools import cache
import re

def main():
    from sys import argv
    with open(argv[1]) as file:
        rules, msgs = read(file)
    #print(rules)

    print('part[1]:', count_acyclic(rules, msgs))
    print('part[2]:', count_cyclic_42_31(rules, msgs))


def count_acyclic(rules, msgs):
    pat = re.compile(generate_re(rules))
    return sum(1 for s in msgs if pat.fullmatch(s))


def count_cyclic_42_31(rules, msgs):
    r42 = generate_re(rules, 42)
    r31 = generate_re(rules, 31)
    if not r42 or not r31:
        return

    # need to match manually extracted rule[0] = 'X+X{n}Y{n}',
    # which regex can't represent, so start w/more relaxed
    full = re.compile(f'{r42}{{2,}}(){r31}+')

    # then manually validate constraint by counting Xs and Ys
    # (which doesn't work in general, but ok here)
    pat42 = re.compile(r42)
    pat31 = re.compile(r31)

    def is_match(s):
        if m := full.fullmatch(s):
            i = m.start(1)  # start of rule 31 sequence
            n42 = count_fullmatches(pat42, s, 0, i)
            n31 = count_fullmatches(pat31, s, i, len(s))
            return n42 > n31

    return sum(1 for s in msgs if is_match(s))


def count_fullmatches(pat, s, pos, end):
    n = 0
    while pos < end:
        if not (m := pat.match(s, pos, end)):
            return
        pos = m.end()
        n += 1
    return n


def generate_re(rules, rule=0):
    @cache
    def gen(rule):
        alts = rules[rule]
        if isinstance(alts, str):
            return alts

        if len(alts) == 1:
            return ''.join(gen(r) for r in alts[0])
        else:
            return '(?:' + '|'.join(
                ''.join(gen(r) for r in a)
                for a in alts
            ) + ')'

    if rule in rules:
        return gen(rule)


def read(file):
    rules = { }
    for line in file:
        line = line.strip()
        if not line: break
        r, e = parse_rule(line)
        rules[r] = e

    return rules, [ line.strip() for line in file ]


def parse_rule(s):
    rule, rhs = s.split(': ')
    if rhs.startswith('"'):
        return int(rule), rhs.strip('"')

    return int(rule), tuple(
        tuple(int(e) for e in a.split())
        for a in rhs.split(' | ')
    )


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def check_acyclic(path, exp):
    rules, msgs = read(open(path))
    pat = re.compile(generate_re(rules))

    for s in msgs:
        assert bool(pat.fullmatch(s)) == (s in exp)

    assert count_acyclic(rules, msgs) == len(exp)


def test1_ex0():
    check_acyclic('ex0.txt', { 'ababbb', 'abbbab' })

def test1_ex1():
    check_acyclic('ex1.txt', {
        'bbabbbbaabaabba',
        'ababaaaaaabaaab',
        'ababaaaaabbbaba',
    })

def test1_answer():
    assert count_acyclic(*read(open('input.txt'))) == 291


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex1():
    rules, msgs = read(open('ex1.txt'))
    exp = {
        'bbabbbbaabaabba',
        'babbbbaabbbbbabbbbbbaabaaabaaa',
        'aaabbbbbbaaaabaababaabababbabaaabbababababaaa',
        'bbbbbbbaaaabbbbaaabbabaaa',
        'bbbababbbbaaaaaaaabbababaaababaabab',
        'ababaaaaaabaaab',
        'ababaaaaabbbaba',
        'baabbaaaabbaaaababbaababb',
        'abbbbabbbbaaaababbbbbbaaaababb',
        'aaaaabbaabaaaaababaa',
        'aaaabbaabbaaaaaaabbbabbbaaabbaabaaa',
        'aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba',
    }
    for s in msgs:
        assert count_cyclic_42_31(rules, (s,)) == (1 if s in exp else 0)

    assert count_cyclic_42_31(rules, msgs) == 12

def test2_answer():
    assert count_cyclic_42_31(*read(open('input.txt'))) == 409
