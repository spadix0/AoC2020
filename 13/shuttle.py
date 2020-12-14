#!/usr/bin/env python3

def main():
    from sys import argv
    with open(argv[1]) as file:
        time, buses, offsets = read(file)
    #print(time, buses, offsets)

    wait, bus = next_bus_after(time, buses)
    print(f'part[1]: bus {bus} after {wait} minutes =>', wait * bus)

    print('part[2]:', find_pattern(buses, offsets))


def next_bus_after(time, buses):
    return min((-time % b, b) for b in buses)


# advance each bus (b) to target schedule offset (-x) in turn.
#
# time is stepped by product of previously considered buses (m)
# to avoid changing their offsets (m % b_k == 0 for each b_k in m).
#
# each time step by m advances current bus by (m % b),
# so we want to find fewest steps j s.t. dt = j * (m%b) % b
# => j = dt / (m%b) % b (where '/' is modular multiplicative inverse)

def find_pattern(buses, offsets):
    t, m = 0, 1
    for b, x in zip(buses, offsets):
        dt = -x - t
        j = dt * pow(m%b, b-2, b) % b
        t += j * m
        assert t % b == -x % b
        m *= b

    return t


def read(file):
    return (
        int(file.readline()),
        *parse_buses(file.readline().strip())
    )


def parse_buses(s):
    return tuple(zip(*[
        (int(n), i)
        for i, n in enumerate(s.split(','))
        if n.isdecimal()
    ]))


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    assert next_bus_after(*read(open('ex0.txt'))[:2]) == (5, 59)

def test1_answer():
    assert next_bus_after(*read(open('input.txt'))[:2]) == (9, 17)


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0():
    assert find_pattern(*read(open('ex0.txt'))[1:]) == 1068781

def test2_ex1():
    assert find_pattern(*parse_buses('17,x,13,19')) == 3417

def test2_ex2():
    assert find_pattern(*parse_buses('67,7,59,61')) == 754018

def test2_ex3():
    assert find_pattern(*parse_buses('67,x,7,59,61')) == 779210

def test2_ex4():
    assert find_pattern(*parse_buses('67,7,x,59,61')) == 1261476

def test2_ex5():
    assert find_pattern(*parse_buses('1789,37,47,1889')) == 1202161486

def test2_answer():
    assert find_pattern(*read(open('input.txt'))[1:]) == 471793476184394
