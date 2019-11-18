import re
import time


def main():
    area = [[0] * 1000 for _ in range(1000)]

    ids = set()

    for id_, x, y, w, h in get_input():
        intersected = False
        for i in range(w):
            for j in range(h):
                if area[x+i][y+j] != 0:
                    intersected = True
                    if area[x+i][y+j] in ids:
                        ids.remove(area[x+i][y+j])

                area[x + i][y + j] = id_

        if not intersected:
            ids.add(id_)

    return ids


def get_input():
    parser = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\n')
    f = open('input/day_3.1.txt')
    for row in f:
        yield [int(x) for x in parser.match(row).groups()]


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
