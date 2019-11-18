import time


def main():
    data = open('input/day_5.1.txt').read().strip()
    # data = 'xxxSbBaAs'

    removed = True
    while removed:
        i = 1
        removed = False

        while i < len(data):
            if data[i].lower() == data[i-1].lower() and data[i] != data[i-1]:
                data = data[:i-1] + data[i+1:]
                removed = True
            else:
                i += 1

    return len(data)


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')