#!/usr/bin/env python3
from collections import defaultdict

def main():
    from sys import argv
    with open(argv[1]) as file:
        prog = read(file)
    #print(prog)

    acc, pc, trace = exec(prog)
    print('part[1]:', acc)
    print('part[2]:', exit_search1(prog, trace)[0])


def read(file):
    return [
        (toks[0], int(toks[1]))
        for line in file
        if (toks := line.strip().split())
    ]


def exec(prog, patch=()):
    pc, acc, vis = 0, 0, set()

    while pc not in vis and 0 <= pc < len(prog):
        vis.add(pc)
        op, arg = prog[pc]

        if op == 'acc':
            acc += arg
        elif (op == 'jmp') != (pc in patch):
            pc += arg - 1
        pc += 1

    return acc, pc, vis


# search by "executing" in reverse from target and testing each instruction.
# linear in length of program (but more memory (still linear))

def exit_search1(prog, trace=None):
    fixpc = find_patch(prog, trace)
    return exec(prog, {fixpc})[0], fixpc


# brute force search by patching each candidate instruction and executing.
# quadratic in length of program (but, even input.txt is only hundreds
# and this only tries necessary cases)

def exit_search2(prog, trace=None):
    for fixpc in trace or exec(prog)[2]:
        if prog[fixpc][0] != 'acc':
            acc, pc, _ = exec(prog, {fixpc})
            if pc == len(prog):
                return acc, fixpc

    assert not 'not found'


def find_patch(prog, trace=None):
    dsts = collect_patch_dsts(prog, trace)
    srcs = collect_jmp_tgts(prog)

    front, vis = { len(prog) }, set()
    while True:
        pc = front.pop()
        if (ppc := dsts.get(pc)) is not None:
            return ppc

        assert pc not in vis
        vis.add(pc)

        front |= srcs[pc] - vis
        if prog[pc-1][0] != 'jmp':
            assert pc-1 not in vis
            front.add(pc-1)


# generate map from patched instruction destination to original trace pc
# (reverse edge candidates that could extend initially reachable set)
def collect_patch_dsts(prog, trace=None):
    return {
        pc + 1 if prog[pc][0] == 'jmp' else pc + prog[pc][1]: pc
        for pc in trace or exec(prog)[2]
        if prog[pc][0] != 'acc'
    }


# generate lookup of jmp targets for reverse execution
def collect_jmp_tgts(prog):
    srcs = defaultdict(set)
    for pc, (op, arg) in enumerate(prog):
        if op == 'jmp' and 0 <= (npc := pc + arg) <= len(prog):
            srcs[npc].add(pc)
    return srcs


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

import pytest

@pytest.fixture
def ex0():
    with open('ex0.txt') as file:
        return read(file)

@pytest.fixture
def input():
    with open('input.txt') as file:
        return read(file)


def test_ex0_read(ex0):
    assert len(ex0) == 9
    for inst in ex0:
        assert len(inst) == 2
        assert inst[0] in { 'acc', 'jmp', 'nop' }
        assert isinstance(inst[1], int)

def test1_ex0(ex0):
    acc, pc, trace = exec(ex0)
    assert acc == 5
    assert pc == 1
    assert trace == { 0, 1, 2, 3, 4, 6, 7 }

def test1_answer(input):
    acc, pc, trace = exec(input)
    assert acc == 1930
    assert pc == 310
    assert len(trace) == 205


#------------------------------------------------------------------------------
# part 2 examples

def test2_ex0_quad(ex0):
    assert exit_search2(ex0) == (8, 7)

def test2_ex0_lin(ex0):
    assert exit_search1(ex0) == (8, 7)

def test2_answer(input):
    assert exit_search2(input) == (1688, 217)
    assert exit_search1(input) == (1688, 217)


@pytest.mark.benchmark(group='tiny')
def testperf2_ex0_quad(ex0, benchmark):
    assert benchmark(exit_search2, ex0) == (8, 7)

@pytest.mark.benchmark(group='tiny')
def testperf2_ex0_linear(ex0, benchmark):
    assert benchmark(exit_search1, ex0) == (8, 7)


@pytest.mark.benchmark(group='nominal')
def testperf2_input_quad(input, benchmark):
    assert benchmark(exit_search2, input) == (1688, 217)

@pytest.mark.benchmark(group='nominal')
def testperf2_input_linear(input, benchmark):
    assert benchmark(exit_search1, input) == (1688, 217)
