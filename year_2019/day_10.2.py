import time
from collections import defaultdict


def main():
    field = data()
    projected_asteroids = get_best_view(field)
    destroy(projected_asteroids)


def get_best_view(field):
    n = len(field)
    m = len(field[0])

    max_visible_asteroids = {}
    for i in range(n):
        for j in range(m):
            if field[i][j] == '#':
                visible_asteroids = count_asteroids(field, i, j)
                if len(visible_asteroids) > len(max_visible_asteroids):
                    max_visible_asteroids = visible_asteroids

    return max_visible_asteroids


def destroy(projected_asteroids):
    projected_asteroids = {k: sorted(v) for k, v in projected_asteroids.items()}
    ordered_in_round = sorted(projected_asteroids.items(), key=sort_directions)

    frags = 0
    i = 0
    while frags < 199:
        ordered_in_round[i][1].pop(0)
        if len(ordered_in_round[i][1]) == 0:
            ordered_in_round.pop(i)
        else:
            i += 1

        if i >= len(ordered_in_round):
            i = 0

        frags += 1

    y, x = ordered_in_round[i][1][0][1]
    print(x * 100 + y)


def sort_directions(v):
    (y, x), value = v
    if x >= 0:
        part = 0
    else:
        part = 1

    if x == 0:
        tg = y * float('inf')
    else:
        tg = y / x

    return part, tg


def count_asteroids(field, x, y):
    n = len(field)
    m = len(field[0])

    directions = defaultdict(list)

    for i in range(n):
        for j in range(m):
            if field[i][j] == '#' and not (i == x and j == y):
                direction_gcd = gcd(abs(i - x), abs(j - y))
                directions[((i - x) // direction_gcd, (j - y) // direction_gcd)].append((abs(i - x), (i, j)))

    return directions


def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a % b)


def data():
    return open('input/day_10.txt').read().strip().split('\n')


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
