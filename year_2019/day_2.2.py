import time
from typing import List


def main():
    source = data()

    for i in range(100):
        for j in range(100):
            code = source[:]

            code[1] = i
            code[2] = j

            k = 0
            while code[k] != 99:
                if code[k] == 1:
                    code[code[k+3]] = code[code[k+1]] + code[code[k+2]]
                elif code[k] == 2:
                    code[code[k+3]] = code[code[k+1]] * code[code[k+2]]
                else:
                    raise Exception()
                k += 4

            if code[0] == 19690720:
                print(i, j)
                return


def data() -> List[int]:
    input_text = open('input/day_2.txt').read()
    return [int(x) for x in input_text.strip().split(',')]


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
