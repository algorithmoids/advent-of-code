import time
from collections import defaultdict
from typing import List, Tuple


def main():
    order = defaultdict(list)
    for f, t in data():
        order[f].append(t)

    print(count('COM', 0, order))


def count(orbit, previous_count, order):
    if orbit not in order:
        return previous_count

    total = previous_count
    for next_orbit in order[orbit]:
        total += count(next_orbit, previous_count + 1, order)

    return total


def data() -> List[List[str]]:
    input_text = open('input/day_6.txt').read()
    return [x.split(')') for x in input_text.strip().split('\n')]


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
