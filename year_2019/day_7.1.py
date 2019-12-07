import time
from typing import List


def main():
    source = data()

    max_output = 0
    for combination in get_combinations(list(range(5))):
        prev_output = 0
        for element in combination:
            code = source[:]
            input = [element, prev_output]
            prev_output = run_program(code, input)[0]

        max_output = max(max_output, prev_output)

    print(max_output)


def get_combinations(elements):
    if not elements:
        yield []

    for i in range(len(elements)):
        for combination in get_combinations(elements[:i] + elements[i+1:]):
            yield [elements[i]] + combination


def run_program(code, input):
    output = []

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
        elif opcode == '05':
            if (code[code[i + 1]] if a == '0' else code[i + 1]) != 0:
                i = code[code[i + 2]] if b == '0' else code[i + 2]
            else:
                i += 3
        elif opcode == '06':
            if (code[code[i + 1]] if a == '0' else code[i + 1]) == 0:
                i = code[code[i + 2]] if b == '0' else code[i + 2]
            else:
                i += 3
        elif opcode == '07':
            code[code[i+3]] = int((code[code[i+1]] if a == '0' else code[i+1]) < (code[code[i+2]] if b == '0' else code[i+2]))
            i += 4
        elif opcode == '08':
            code[code[i+3]] = int((code[code[i+1]] if a == '0' else code[i+1]) == (code[code[i+2]] if b == '0' else code[i+2]))
            i += 4
        elif opcode == '99':
            break
        else:
            raise Exception()

    return output


def parse_command(command):
    *_, c, b, a, s1, s2 = '0000' + str(command)
    return s1 + s2, a, b, c


def data() -> List[int]:
    input_text = open('input/day_7.txt').read()
    return [int(x) for x in input_text.strip().split(',')]


if __name__ == '__main__':
    tstart = time.monotonic()
    main()
    time_spent = time.monotonic() - tstart
    print(f'took: {time_spent:.3f} s')
