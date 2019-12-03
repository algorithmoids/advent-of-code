import time
from typing import List


def main():
    wires = data()

    first_wire = {}
    position = (0, 0)
    length = 0
    for line in wires[0]:
        for steps in range(int(line[1:])):
            position = step(line[0], position)
            length += 1
            first_wire[position] = length

    position = (0, 0)
    intersections = list()
    length = 0
    for line in wires[1]:
        for steps in range(int(line[1:])):
            position = step(line[0], position)
            length += 1
            if position in first_wire:
                intersections.append(first_wire[position] + length)

    closest_intersection = min(intersections)
    print(closest_intersection)


def step(direction, position):
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
