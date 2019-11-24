import time


def main():
    numbers = data().split('\n')
    total = 0
    for number in numbers:
        if not number:
            continue
        if number[0] == '-':
            total -= int(number[1:])
        else:
            total += int(number[1:])
    return total


def data():
    return open('input/day_1.1.txt').read()


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')