#!/usr/bin/env python3
import re

def main():
    from sys import argv
    with open(argv[1]) as file:
        expr = read(file)
    #print(expr)

    print('part[1]:', sum(Parser(e).eval() for e in expr))
    print('part[2]:', sum(Parser(e, {'+': 2}).eval() for e in expr))


def read(file):
    return [ tokenize(line.strip()) for line in file ]


def tokenize(s):
    return [
        int(t) if t.isdecimal() else t
        for t in re.split(r'\s+|(?<=[(])|(?=[)])', s)
    ]


class Parser:
    def __init__(self, toks, bind={}):
        self.toks = iter(toks)
        self.bind = {'+': 1, '*': 1} | bind
        self.lookahead = None

    def next(self):
        t = self.lookahead
        if t is None:
            try:
                t = next(self.toks)
            except StopIteration:
                pass
        else:
            self.lookahead = None
        return t

    def peek(self):
        t = self.lookahead
        if t is None:
            t = self.lookahead = self.next()
        return t

    def eval(self, bind=0):
        t = self.next()
        if t == '(':
            val = self.eval()
            t = self.next()
            assert t == ')', t
        else:
            assert isinstance(t, int), t
            val = t

        while (b := self.bind.get(self.peek(), 0)) > bind:
            t = self.next()
            if t == '*':
                val *= self.eval(b)
            elif t == '+':
                val += self.eval(b)
            else:
                assert t is None # EoS
                break

        return val


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    for e, v in zip(read(open('ex0.txt')), [
        71, 51, 26, 437, 12240, 13632
    ]):
        assert Parser(e).eval() == v

def test1_answer():
    expr = read(open('input.txt'))
    assert sum(Parser(e).eval() for e in expr) == 12956356593940


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    for e, v in zip(read(open('ex0.txt')), [
        231, 51, 46, 1445, 669060, 23340
    ]):
        assert Parser(e, {'+': 2}).eval() == v

def test2_answer():
    expr = read(open('input.txt'))
    assert sum(Parser(e, {'+': 2}).eval() for e in expr) == 94240043727614
