import time


def main():
    start = 137683
    end = 596253

    passwords = 0

    for v in range(start, end):
        sv = str(v)
        decrease = False
        has_pair = False
        for i in range(5):
            if sv[i] > sv[i+1]:
                decrease = True
                break
            if sv[i] == sv[i+1]:
                has_pair = True

        if not decrease and has_pair:
            passwords += 1

    print(passwords)


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
