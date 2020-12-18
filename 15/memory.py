#!/usr/bin/env python3

def main():
    from sys import argv
    with open(argv[1]) as file:
        data = parse(file.read())
    #print(data)

    game = Game(data)
    print('part[1]:', game.play_until(2020))
    print('part[2]:', game.play_until(30_000_000))


class Game:
    def __init__(self, data):
        self.seed(data)

    def seed(self, data):
        mem = self.mem = { }
        for t, n in enumerate(data, 1):
            tp = mem.get(n, t)
            mem[n] = t

        self.prev_turn = tp
        self.turn = t

    def play_until(self, turn):
        mem, t, tp, n = self.mem, self.turn, self.prev_turn, None

        while t < turn:
            n = t - tp
            t += 1
            tp = mem.get(n, t)
            mem[n] = t

        self.prev_turn = tp
        self.turn = t

        return n


def parse(s):
    return [ int(n) for n in s.split(',') ]


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def check1(s, exp):
    assert Game(parse(s)).play_until(2020) == exp


def test1_ex0():
    check1(open('ex0.txt').read(), 436)

def test1_ex1():
    check1('1,3,2', 1)

def test1_ex2():
    check1('2,1,3', 10)

def test1_ex3():
    check1('1,2,3', 27)

def test1_ex4():
    check1('2,3,1', 78)

def test1_ex5():
    check1('3,2,1', 438)

def test1_ex6():
    check1('3,1,2', 1836)

def test1_answer():
    check1(open('input.txt').read(), 234)


#------------------------------------------------------------------------------
# part 2 examples

def check2(s, exp):
    assert Game(parse(s)).play_until(30_000_000) == exp


def test2_ex0():
    check2(open('ex0.txt').read(), 175594)

def test2_ex1():
    check2('1,3,2', 2578)

def test2_ex2():
    check2('2,1,3', 3544142)

def test2_ex3():
    check2('1,2,3', 261214)

def test2_ex4():
    check2('2,3,1', 6895259)

def test2_ex5():
    check2('3,2,1', 18)

def test2_ex6():
    check2('3,1,2', 362)

def test2_answer():
    check2(open('input.txt').read(), 8984)
