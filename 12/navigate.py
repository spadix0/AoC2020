#!/usr/bin/env python3

def main():
    from sys import argv
    with open(argv[1]) as file:
        path = read(file)
    #print(path)

    p1 = traverse(path)
    print(p1, sum(abs(x) for x in p1))

    p2 = waypoint(path)
    print(p2, sum(abs(x) for x in p2))


def read(file):
    return [
        (line[0].upper(), int(line[1:]))
        for line in file
    ]


def traverse(path):
    x, y, dx, dy = 0, 0, 1, 0

    dispatch = {
        'N': lambda a: (x, y+a, dx, dy),
        'S': lambda a: (x, y-a, dx, dy),
        'E': lambda a: (x+a, y, dx, dy),
        'W': lambda a: (x-a, y, dx, dy),
        'L': lambda a: (x, y, *rotate(dx, dy, -a)),
        'R': lambda a: (x, y, *rotate(dx, dy, a)),
        'F': lambda a: (x + a*dx, y + a*dy, dx, dy),
    }

    for act, a in path:
        x, y, dx, dy = dispatch[act](a)

    return x, y


def waypoint(path):
    x, y, dx, dy = 0, 0, 10, 1

    dispatch = {
        'N': lambda a: (x, y, dx, dy+a),
        'S': lambda a: (x, y, dx, dy-a),
        'E': lambda a: (x, y, dx+a, dy),
        'W': lambda a: (x, y, dx-a, dy),
        'L': lambda a: (x, y, *rotate(dx, dy, -a)),
        'R': lambda a: (x, y, *rotate(dx, dy, a)),
        'F': lambda a: (x + a*dx, y + a*dy, dx, dy),
    }

    for act, a in path:
        x, y, dx, dy = dispatch[act](a)

    return x, y


def rotate(dx, dy, deg):
    a = (deg//90 + 4) % 4
    s = 1 - abs(1 - a)
    c = abs(2 - a) - 1
    assert s in (-1, 0, 1)
    assert c in (-1, 0, 1)
    return c*dx + s*dy, c*dy - s*dx


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test_rotate():
    from math import pi, sin, cos
    for deg in range(-270, 270+1, 90):
        r = pi/180 * deg
        assert rotate(0, 1, deg) == (round(sin(r)), round(cos(r)))

def test1_ex0():
    assert traverse(read(open('ex0.txt'))) == (17, -8)

def test_answer1():
    assert traverse(read(open('input.txt'))) == (-163, -218)


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    assert waypoint(read(open('ex0.txt'))) == (214, -72)

def test_answer2():
    assert waypoint(read(open('input.txt'))) == (-11946, -16645)
