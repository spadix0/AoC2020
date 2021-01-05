#!/usr/bin/env python3
from itertools import count

M = 20201227

def main():
    from sys import argv
    with open(argv[1]) as file:
        pub = [ int(line) for line in file ]
    #print(pub)

    loop0 = reverse_loop(7, pub[0])
    print('card loop size:', loop0)

    loop1 = reverse_loop(7, pub[1])
    print('door loop size:', loop1)

    key = pow(pub[1], loop0, M)
    print('encryption key:', key)

    assert key == pow(pub[0], loop1, M)


def reverse_loop(subj, tgt):
    val = 1
    for loop in count(1):
        val = val * subj % M
        if val == tgt:
            return loop


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
def test_ex0_card_loop():
    assert reverse_loop(7, 5764801) == 8

def test_ex0_door_loop():
    assert reverse_loop(7, 17807724) == 11

def test_ex0_key():
    assert pow(5764801, 11, M) == 14897079
    assert pow(17807724, 8, M) == 14897079

def test_answer():
    pub0, pub1 = (int(line) for line in open('input.txt'))

    loop0 = reverse_loop(7, pub0)
    assert loop0 == 2232839

    loop1 = reverse_loop(7, pub1)
    assert loop1 == 529361

    assert pow(pub1, loop0, M) == 11328376
    assert pow(pub0, loop1, M) == 11328376
