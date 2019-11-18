import time


def main():
    ids = list(rows())
    for i in range(len(ids)):
        A = ids[i]
        for j in range(i+1, len(ids)):
            B = ids[j]
            diff = [a for a, b in zip(A, B) if a == b]
            if len(diff) == len(A) - 1:
                return ''.join(diff)


def rows():
    f = open('input/day_2.1.txt')
    for row in f:
        yield row.strip()


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
