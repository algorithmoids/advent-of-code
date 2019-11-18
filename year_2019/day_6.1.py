import re
import time
from collections import Counter
from typing import List


def main():
    points = get_points()

    N = max([x[0] for x in points])
    M = max([x[1] for x in points])

    field = [[-1] * M for _ in range(N)]

    for i in range(N):
        for j in range(M):
            closest_distance = abs(points[0][0] - i) + abs(points[0][1] - j)
            closest_point = 0
            equal = False

            for point_id, point in enumerate(points[1:], 1):
                distance = abs(point[0] - i) + abs(point[1] - j)
                if distance < closest_distance:
                    closest_point = point_id
                    closest_distance = distance
                    equal = False
                elif distance == closest_distance:
                    equal = True

            if not equal:
                field[i][j] = closest_point

    infinite = set(field[0]).union(field[-1]).union(list(zip(*field))[0]).union(list(zip(*field))[-1]).union([None])

    counters = Counter()
    for i in range(N):
        for j in range(M):
            if field[i][j] not in infinite:
                counters[field[i][j]] += 1

    print(counters)



def get_points() -> List[List[int]]:
    parser = re.compile(r'(\d+), (\d+)\n')
    data = open('input/day_6.1.txt').readlines()
    return [list(map(int, parser.match(x).groups())) for x in data]


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')