import time


def main():
    field = data()
    n = len(field)
    m = len(field[0])

    max_asteroids = 0
    for i in range(n):
        for j in range(m):
            if field[i][j] == '#':
                max_asteroids = max(count_asteroids(field, i, j), max_asteroids)

    print(max_asteroids)


def count_asteroids(field, x, y):
    n = len(field)
    m = len(field[0])

    directions = set()

    for i in range(n):
        for j in range(m):
            if field[i][j] == '#' and not (i == x and j == y):
                direction_gcd = gcd(abs(i - x), abs(j - y))
                directions.add(((i - x) // direction_gcd, (j - y) // direction_gcd))

    return len(directions)


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
