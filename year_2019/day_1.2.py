import time


def main():
    reached = set()

    total = 0
    for number in numbers():
        total += number
        if total in reached:
            return total
        reached.add(total)


def numbers():
    while True:
        f = open('input/day_1.1.txt')
        for row in f:
            if row[0] == '-':
                yield -int(row[1:])
            else:
                yield int(row[1:])


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
