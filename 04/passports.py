#!/usr/bin/env python3
import re

ALL_FIELDS = set('byr iyr eyr hgt hcl ecl pid cid'.split())
REQ_FIELDS = ALL_FIELDS - {'cid'}

EYE_COLORS = set('amb blu brn gry grn hzl oth'.split())

hgt_re = re.compile(r'^([0-9]+)(cm|in)$')
hcl_re = re.compile(r'^#[0-9a-f]{6}$')
pid_re = re.compile(r'^[0-9]{9}$')


def main():
    from sys import argv
    with open(argv[1]) as file:
        entries = parse_batch(file.read())

    print('part[1]:', count_valid(entries, validate_keys))
    print('part[2]:', count_valid(entries, validate_entry))


def parse_batch(batch):
    return [
        {
            kv[0]: kv[1]
            for field in entry.split()
            if (kv := field.split(':'))
        }
        for entry in batch.split('\n\n')
    ]


def count_valid(entries, validator):
    return sum(1 for ent in entries if validator(ent))


# validator for part1
def validate_keys(ent):
    return REQ_FIELDS <= ent.keys() <= ALL_FIELDS


# validator for part2
def validate_entry(ent):
    return validate_keys(ent) and \
        all(validate_field(k, v) for k, v in ent.items())


def validate_field(k, v):
    try:
        return field_validators[k](v)
    except ValueError:
        pass
    except KeyError:
        pass


def validate_hgt(v):
    if m := hgt_re.match(v):
        n = int(m[1])
        if m[2] == 'cm':
            return 150 <= n <= 193
        else:
            return 59 <= n <= 76


field_validators = {
    'byr': lambda v: 1920 <= int(v) <= 2002,
    'iyr': lambda v: 2010 <= int(v) <= 2020,
    'eyr': lambda v: 2020 <= int(v) <= 2030,
    'hgt': validate_hgt,
    'hcl': hcl_re.match,
    'ecl': lambda v: v in EYE_COLORS,
    'pid': pid_re.match,
    'cid': lambda v: True,
}


if __name__ == '__main__':
    main()


#------------------------------------------------------------------------------
# part 1 examples

def test1_ex0():
    ex0 = parse_batch(open('ex0.txt').read())

    assert len(ex0) == 4
    assert len(ex0[0]) == 8
    assert validate_keys(ex0[0])

    assert len(ex0[1]) == 7
    assert not validate_keys(ex0[1])

    assert len(ex0[2]) == 7
    assert validate_keys(ex0[2])

    assert len(ex0[3]) == 6
    assert not validate_keys(ex0[3])

    assert count_valid(ex0, validate_keys) == 2

def test1_answer():
    input = parse_batch(open('input.txt').read())
    assert count_valid(input, validate_keys) == 216


#------------------------------------------------------------------------------
# part 2 examples

EX_FIELDS = '''\
byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789
'''

def test2_fields():
    cases = [ case.split() for case in EX_FIELDS.split('\n') if case ]
    for field, exp, val in cases:
        assert exp.startswith('valid') == bool(validate_field(field, val))

def test2_ex1_invalid():
    for ent in parse_batch(open('ex1_invalid.txt').read()):
        assert not validate_entry(ent)

def test2_ex2_valid():
    for ent in parse_batch(open('ex2_valid.txt').read()):
        assert validate_entry(ent)

def test2_answer():
    input = parse_batch(open('input.txt').read())
    assert count_valid(input, validate_entry) == 150
