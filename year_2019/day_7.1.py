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

    def data(index, mode):
        return code[code[index]] if mode == '0' else code[index]

    output = []

    i = 0
    while True:
        opcode, a, b = parse_command(code[i])
        if opcode == '01':
            code[code[i+3]] = data(i+1, a) + data(i+2, b)
            i += 4
        elif opcode == '02':
            code[code[i+3]] = data(i+1, a) * data(i+2, b)
            i += 4
        elif opcode == '03':
            code[code[i+1]] = input.pop(0)
            i += 2
        elif opcode == '04':
            output.append(code[code[i+1]])
            i += 2
        elif opcode == '05':
            if (data(i+1, b)) != 0:
                i = data(i+2, b)
            else:
                i += 3
        elif opcode == '06':
            if (data(i + 1, b)) == 0:
                i = data(i+2, b)
            else:
                i += 3
        elif opcode == '07':
            code[code[i+3]] = int(data(i+1, a) < data(i+2, b))
            i += 4
        elif opcode == '08':
            code[code[i+3]] = int(data(i+1, a) == data(i+2, b))
            i += 4
        elif opcode == '99':
            break
        else:
            raise Exception()

    return output


def parse_command(command):
    *_, b, a, s1, s2 = '000' + str(command)
    return s1 + s2, a, b


def data() -> List[int]:
    input_text = open('input/day_7.txt').read()
    return [int(x) for x in input_text.strip().split(',')]


if __name__ == '__main__':
    tstart = time.monotonic()
    main()
    time_spent = time.monotonic() - tstart
    print(f'took: {time_spent:.3f} s')
