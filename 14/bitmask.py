#!/usr/bin/env python3
from array import array
from itertools import combinations
from functools import cache		# NB new in 3.9
from collections import namedtuple

def main():
    from sys import argv
    with open(argv[1]) as file:
        prog = read(file)
    #print(prog)
    #print(len(prog), sum(len(m.writes) for m in prog))

    print('part[1]:', sum(exec_datamask(prog).values()))
    print('part[2]:', exec_addrmask_bdd(prog).sum())


class Write(namedtuple('Write', 'addr data')):
    def __repr__(self):
        return f'[{self.addr}]={self.data}'


class MaskWrites:
    __slots__ = 'maskx', 'mask1', 'writes'

    def __init__(self, maskx, mask1):
        self.maskx = maskx
        self.mask1 = mask1
        self.writes = [ ]

    def __repr__(self):
        writes = ' '.join(str(w) for w in self.writes)
        return f'&{self.maskx:036b}|{self.mask1:036b} [{writes}]'

    TR_MASKX = str.maketrans('1X', '01')
    TR_MASK1 = str.maketrans('X', '0')

    @classmethod
    def parse(cls, s):
        maskx = int(s.translate(cls.TR_MASKX), 2)
        mask1 = int(s.translate(cls.TR_MASK1), 2)
        return cls(maskx, mask1)


def read(file):
    prog = [ ]
    for line in file:
        lhs, rhs = line.strip().split(' = ')
        if lhs == 'mask':
            prog.append(MaskWrites.parse(rhs))
        else:
            prog[-1].writes.append(
                Write(int(lhs[4:-1]), int(rhs)))

    return prog


def exec_datamask(prog):
    return {
        addr: data & m.maskx | m.mask1
        for m in prog
        for addr, data in m.writes
    }


#------------------------------------------------------------------------------
# 3 alternative implementations for part 2:

# splat – combinatorial expansion
#
# initial, obvious approach that expands and writes all possible combinations
# of masked address bits to sparse memory map.  simple and effective for
# provided problem input.  explodes and runs out of memory if any mask has
# too many Xs, such as example from part 1 (ex0)

def exec_addrmask_splat(prog):
    mem = { }
    for m in prog:
        maskx, mask1 = m.maskx, m.mask1
        bits = [ 1<<b for b in range(36) if maskx>>b & 1 ]
        maskz = ~maskx
        for n in range(len(bits)+1):
            for c in combinations(bits, n):
                off = mask1 | sum(c)
                for addr, data in m.writes:
                    mem[addr & maskz | off] = data

    return mem


# split – hypercube intersection
#
# tracks masked addresses and only expands overwritten regions.  minimal
# memory usage even w/many Xs but can still explode for other cases.
# brute force intersection checks are too slow to be useful, even with
# nominal input.

def exec_addrmask_split(prog):
    mem = [ ]
    for m in prog:
        for addr, data in m.writes:
            addr = addr & ~m.maskx | m.mask1
            mem = _split_isect(mem, m.maskx, addr)
            mem.append((m.maskx, addr, data))

    return mem


def _split_isect(prev, mask, addr):
    next = [ ]
    for p in prev:
        m, a, d = p
        fixed = ~(mask | m)
        if a & fixed == addr & fixed:
            keep = m & ~mask
            bits = [ 1<<b for b in range(36) if keep>>b & 1 ]
            for split in powersum(bits):
                na = split | (a & ~keep)
                if na & ~mask != addr:
                    next.append((mask & m, na, d))
        else:
            next.append(p)

    return next


def powersum(s):
    return (
        sum(c)
        for r in range(len(s)+1)
        for c in combinations(s, r)
    )


def split_sum(mem):
    return sum(
        v << f'{m:b}'.count('1')
        for m, a, v in mem
    )


# BDD - modified binary decision diagram
#
# with multiple leaf/value nodes for write data.  questionable compromise
# between speed and memory with support for arbitrary masks.  needs optimized.

def exec_addrmask_bdd(prog):
    mem = BDD()
    for m in prog:
        for addr, data in m.writes:
            mem.write(m.maskx, addr & ~m.maskx | m.mask1, data)

    return mem


class BDD:
    Node = namedtuple('Node', 'bit i0 i1')
    Const = namedtuple('Const', 'bit val')

    def __init__(self):
        self.nodes = [ ]
        self.nodemap = { }
        self.root = self._const(0)

    def __repr__(self):
        nodes, vis = self.nodes, set()

        def rep_r(i):
            n = nodes[i]
            if n.bit > 35: return f'({n.val})'
            if i in vis: return f'[{i}@{n.bit}..]'
            vis.add(i)
            s0, s1 = rep_r(n.i0), rep_r(n.i1)
            return f'[{i}@{n.bit} {s0} {s1}]'

        return rep_r(self.root)

    def __len__(self):
        return len(self.nodes)

    def count(self):
        nodes, vis = self.nodes, set()

        def cnt_r(i):
            n = nodes[i]
            if n.bit > 35 or i in vis: return 0
            vis.add(i)
            return 1 + cnt_r(n.i0) + cnt_r(n.i1)

        return cnt_r(self.root)

    def sum(self):
        nodes, memo = self.nodes, { }

        def sum_r(i, b):
            n = nodes[i]
            if n.bit > 35:
                v = n.val
            elif (v := memo.get(i)) is None:
                v = sum_r(n.i0, n.bit+1) + sum_r(n.i1, n.bit+1)
                memo[i] = v
            return v << n.bit-b

        return sum_r(self.root, 0)

    def _require(self, n):
        if (i := self.nodemap.get(n)) is None:
            nodes = self.nodes
            i = self.nodemap[n] = len(nodes)
            nodes.append(n)
        return i

    def _const(self, val):
        return self._require(self.Const(36, val))  # terminals at "bit" 36

    def _node(self, bit, i0, i1):
        if i0 == i1:
            return i0
        return self._require(self.Node(bit, i0, i1))

    def write(self, mask, addr, data):
        nodes, node = self.nodes, self._node
        idata = self._const(data)

        @cache
        def wr_r(bit, i):
            if bit > 35:
                return idata

            n = nodes[i]
            assert n.bit >= bit
            if n.bit > bit:
                j = wr_r(bit+1, i)
                if mask>>bit & 1:
                    pass
                elif addr>>bit & 1:
                    j = node(bit, i, j)
                else:
                    j = node(bit, j, i)
            else:
                if mask>>bit & 1:
                    j = node(bit, wr_r(bit+1, n.i0), wr_r(bit+1, n.i1))
                elif addr>>bit & 1:
                    j = node(bit, n.i0, wr_r(bit+1, n.i1))
                else:
                    j = node(bit, wr_r(bit+1, n.i0), n.i1)
            return j

        self.root = wr_r(0, self.root)


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
def ex1():
    with open('ex1.txt') as file:
        return read(file)

@pytest.fixture
def input():
    with open('input.txt') as file:
        return read(file)


def test1_ex0(ex0):
    assert exec_datamask(ex0) == { 7: 101, 8: 64 }

def test1_answer(input):
    assert sum(exec_datamask(input).values()) == 12135523360904


#------------------------------------------------------------------------------
# part 2 examples

@pytest.mark.benchmark(group='ex1')
def test2_ex1_splat(ex1, benchmark):
    mem = benchmark(exec_addrmask_splat, ex1)
    assert len(mem) == 10
    assert sum(mem.values()) == 208

@pytest.mark.benchmark(group='input')
def test2_answer_splat(input, benchmark):
    mem = benchmark(exec_addrmask_splat, input)
    assert len(mem) == 77076
    assert sum(mem.values()) == 2741969047858


@pytest.mark.benchmark(group='ex0')
def test2_ex0_split(ex0, benchmark):
    mem = benchmark(exec_addrmask_split, ex0)
    assert len(mem) == 2
    assert split_sum(mem) == 1735166787584

@pytest.mark.benchmark(group='ex1')
def test2_ex1_split(ex1, benchmark):
    mem = benchmark(exec_addrmask_split, ex1)
    assert len(mem) == 2
    assert split_sum(mem) == 208

@pytest.mark.benchmark(group='input')
def test2_answer_split(input, benchmark):
    mem = benchmark(exec_addrmask_split, input)
    assert len(mem) == 9284
    assert split_sum(mem) == 2741969047858


@pytest.mark.benchmark(group='ex0')
def test2_ex0_bdd(ex0, benchmark):
    mem = benchmark(exec_addrmask_bdd, ex0)
    assert len(mem) == 8
    assert mem.count() == 2
    assert mem.sum() == 1735166787584

@pytest.mark.benchmark(group='ex1')
def test2_ex1_bdd(ex1, benchmark):
    mem = benchmark(exec_addrmask_bdd, ex1)
    assert len(mem) == 75
    assert mem.count() == 68
    assert mem.sum() == 208

@pytest.mark.benchmark(group='input')
def test2_answer_bdd(input, benchmark):
    mem = benchmark(exec_addrmask_bdd, input)
    assert len(mem) == 30736
    assert mem.count() == 15435
    assert mem.sum() == 2741969047858
