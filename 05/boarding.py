#!/usr/bin/env python3

def main():
    from sys import argv
    with open(argv[1]) as file:
        seats = read(file)

    print('part[1]:', max(seats))
    print('part[2]:', empty_interior_seat(seats))


def read(file):
    return { seat_id(seat_pos(line)) for line in file }


FB_BIN = str.maketrans('FB', '01')
LR_BIN = str.maketrans('LR', '01')

def seat_pos(bsp):
    return (
        int(bsp[:7].translate(FB_BIN), 2),
        int(bsp[7:].translate(LR_BIN), 2),
    )


def seat_id(pos):
    return 8*pos[0] + pos[1]


def empty_interior_seat(seats):
    # interior range specified somewhat inconsistently as:
    # "Your seat wasn't at the very front or back, though;
    #  the seats with IDs +1 and -1 from yours will be in your list."
    # => checking both because it doesn't matter for this input
    idmin, idmax = min(seats), max(seats)

    # front most and back most row
    rmin, rmax = idmin//8, idmax//8

    # interior range
    inmin = max(8*(rmin + 1), idmin+1)
    inmax = min(8*rmax - 1, idmax-1)
    interior = range(inmin, inmax+1)

    # the one seat
    mt = { i for i in interior if i not in seats }
    assert len(mt) == 1
    return mt.pop()


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex():
    cases = [
        ('FBFBBFFRLR', (44, 5), 357),
        ('BFFFBBFRRR', (70, 7), 567),
        ('FFFBBBFRRR', (14, 7), 119),
        ('BBFFBBFRLL', (102, 4), 820),
    ]
    for bsp, pos, id in cases:
        assert pos == seat_pos(bsp)
        assert id == seat_id(pos)

def test1_answer():
    input = read(open('input.txt'))
    assert max(input) == 848


def test2_not_front():
    # empty seat in front row that has seat IDs +1 and -1
    seats = {
            17,     19, 20, 21, 22, 23,	# *not* it
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35,     37, 38, 39,	# should be this one
        40, 41, 42, 43, 44
    }
    assert empty_interior_seat(seats) == 36

def test2_not_back():
    seats = {
            17, 18, 19, 20, 21, 22, 23,
        24, 25, 26,     28, 29, 30, 31,	# should be this one
        32, 33, 34, 35, 36, 37, 38, 39,
            41, 42, 43, 44		# *not* this one
    }
    assert empty_interior_seat(seats) == 27

def test2_front_edge():
    seats = {
                                    23,
            25, 26, 27, 28, 29, 30, 31,
        32,
    }
    assert empty_interior_seat(seats) == 24

def test2_back_edge():
    seats = {
                                    23,
        24, 25, 26, 27, 28, 29, 30,
        32,
    }
    assert empty_interior_seat(seats) == 31

def test2_answer():
    input = read(open('input.txt'))
    assert empty_interior_seat(input) == 682
