#!/usr/bin/env python3
from math import pi, inf, ceil, floor, gcd, hypot, atan2, sqrt, sin, cos
from array import array
from itertools import *
from functools import partial, reduce
from operator import add, mul, or_
from collections import namedtuple, defaultdict, deque, Counter
from heapq import heapify, heappush, heappop
from io import StringIO
from hashlib import md5
import re, json

zet = frozenset
chainit = chain.from_iterable


def read(file):
    data = [ ]
    for line in file:
        line = line.strip()

    return data


def main():
    from sys import argv, stdin
    with open(argv[1]) as file:
        data = read(file)
    print(data)


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples


#------------------------------------------------------------------------------
# part 2 examples
