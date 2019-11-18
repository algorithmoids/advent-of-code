import re
import time


def main():
    area = [[0] * 1000 for _ in range(1000)]

    for x, y, w, h in get_input():
        for i in range(w):
            for j in range(h):
                area[x+i][y+j] += 1

    return sum([1 for row in area for x in row if x > 1])


def get_input():
    parser = re.compile(r'#\d+ @ (\d+),(\d+): (\d+)x(\d+)\n')
    f = open('input/day_3.1.txt')
    for row in f:
        yield [int(x) for x in parser.match(row).groups()]


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
