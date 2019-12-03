import time
from typing import List


def main():
    wires = data()

    first_wire = set(trace(wires[0]))
    intersections = [x for x in trace(wires[1]) if x in first_wire]
    print(min([abs(x) + abs(y) for x, y in intersections]))


def trace(wire):
    position = (0, 0)
    for line in wire:
        for steps in range(int(line[1:])):
            position = move(line[0], position)
            yield position


def move(direction, position):
    if direction == 'L':
        return position[0] + 1, position[1]
    if direction == 'R':
        return position[0] - 1, position[1]
    if direction == 'U':
        return position[0], position[1] + 1
    if direction == 'D':
        return position[0], position[1] - 1


def data() -> List[List[str]]:
    input_text = open('input/day_3.txt').read()
    return [x.split(',') for x in input_text.strip().split('\n')]


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')

# 27330