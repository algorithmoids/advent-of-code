import time
from typing import List


def main():
    input = [1]
    output = []

    code = data()

    i = 0
    while True:
        opcode, a, b, c = parse_command(code[i])
        if opcode == '01':
            code[code[i+3]] = (code[code[i+1]] if a == '0' else code[i+1]) + (code[code[i+2]] if b == '0' else code[i+2])
            i += 4
        elif opcode == '02':
            code[code[i+3]] = (code[code[i+1]] if a == '0' else code[i+1]) * (code[code[i+2]] if b == '0' else code[i+2])
            i += 4
        elif opcode == '03':
            code[code[i+1]] = input.pop(0)
            i += 2
        elif opcode == '04':
            output.append(code[code[i+1]])
            i += 2
        elif opcode == '99':
            break
        else:
            raise Exception()

    print(output)


def parse_command(command):
    *_, c, b, a, s1, s2 = '0000' + str(command)
    return s1 + s2, a, b, c


def data() -> List[int]:
    input_text = open('input/day_5.txt').read()
    return [int(x) for x in input_text.strip().split(',')]


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
