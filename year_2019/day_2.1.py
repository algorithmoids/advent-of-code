import time
from typing import List


def main():
    code = data()
    code[1] = 12
    code[2] = 2

    i = 0
    while code[i] != 99:
        if code[i] == 1:
            code[code[i+3]] = code[code[i+1]] + code[code[i+2]]
        elif code[i] == 2:
            code[code[i+3]] = code[code[i+1]] * code[code[i+2]]
        else:
            raise Exception()
        i += 4

    print(code[0])


def data() -> List[int]:
    input_text = open('input/day_2.txt').read()
    return [int(x) for x in input_text.strip().split(',')]


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
