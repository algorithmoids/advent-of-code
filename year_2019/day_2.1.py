import time
from collections import defaultdict


def main():
    two = 0
    three = 0

    for row in rows():
        counters = defaultdict(int)
        for letter in row:
            counters[letter] += 1

        if 2 in counters.values():
            two += 1
        if 3 in counters.values():
            three += 1

    return two * three


def rows():
    f = open('input/day_2.1.txt')
    for row in f:
        yield row


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
