import re
import time
from collections import Counter
from typing import List


def main():
    region = 10_000
    points = get_points()

    N = max([x[0] for x in points])
    M = max([x[1] for x in points])

    good_points = 0

    for i in range(-100, N + 100):
        for j in range(-100, M + 100):
            distance = 0
            for point in points:
                distance += abs(point[0] - i) + abs(point[1] - j)

            if distance < region:
                good_points += 1

    print(good_points)



def get_points() -> List[List[int]]:
    parser = re.compile(r'(\d+), (\d+)\n')
    data = open('input/day_6.1.txt').readlines()
    return [list(map(int, parser.match(x).groups())) for x in data]


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
