import time


def main():
    total_fuel = 0

    for mass in data():
        total_fuel += int(mass // 3) - 2

    print(total_fuel)


def data():
    for row in open('input/day_1.txt'):
        yield int(row.strip())


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
