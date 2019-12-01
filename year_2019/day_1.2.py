import time


def main():
    total_fuel = 0

    for mass in data():
        total_fuel += count_fuel(mass)

    print(total_fuel)


def count_fuel(mass):
    fuel = int(mass // 3) - 2
    if fuel <= 0:
        return 0
    else:
        return fuel + count_fuel(fuel)


def data():
    for row in open('input/day_1.txt'):
        yield int(row.strip())


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
