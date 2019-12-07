import time
from queue import Queue
from threading import Thread
from typing import List


class Amplifier(Thread):

    def __init__(self, code, input, output, last_output=None):
        super().__init__()

        self.code = code
        self.input = input
        self.output = output
        self.last_output = last_output

    def run(self):
        i = 0
        while True:
            opcode, a, b, c = parse_command(self.code[i])
            if opcode == '01':
                self.code[self.code[i + 3]] = (self.code[self.code[i + 1]] if a == '0' else self.code[i + 1]) + (
                    self.code[self.code[i + 2]] if b == '0' else self.code[i + 2])
                i += 4
            elif opcode == '02':
                self.code[self.code[i + 3]] = (self.code[self.code[i + 1]] if a == '0' else self.code[i + 1]) * (
                    self.code[self.code[i + 2]] if b == '0' else self.code[i + 2])
                i += 4
            elif opcode == '03':
                self.code[self.code[i + 1]] = self.input.get()
                i += 2
            elif opcode == '04':
                self.output.put(self.code[self.code[i + 1]])
                if self.last_output:
                    self.last_output[0] = self.code[self.code[i + 1]]
                i += 2
            elif opcode == '05':
                if (self.code[self.code[i + 1]] if a == '0' else self.code[i + 1]) != 0:
                    i = self.code[self.code[i + 2]] if b == '0' else self.code[i + 2]
                else:
                    i += 3
            elif opcode == '06':
                if (self.code[self.code[i + 1]] if a == '0' else self.code[i + 1]) == 0:
                    i = self.code[self.code[i + 2]] if b == '0' else self.code[i + 2]
                else:
                    i += 3
            elif opcode == '07':
                self.code[self.code[i + 3]] = int((self.code[self.code[i + 1]] if a == '0' else self.code[i + 1]) < (
                    self.code[self.code[i + 2]] if b == '0' else self.code[i + 2]))
                i += 4
            elif opcode == '08':
                self.code[self.code[i + 3]] = int((self.code[self.code[i + 1]] if a == '0' else self.code[i + 1]) == (
                    self.code[self.code[i + 2]] if b == '0' else self.code[i + 2]))
                i += 4
            elif opcode == '99':
                return
            else:
                raise Exception()


def main():
    source = data()

    max_output = 0
    for combination in get_combinations(list(range(5, 10))):
        q = [Queue() for _ in range(5)]
        for i, element in enumerate(combination):
            q[i].put(element)
        q[0].put(0)

        last_value = [0]
        amplifiers = [
            Amplifier(source[:], q[0], q[1]),
            Amplifier(source[:], q[1], q[2]),
            Amplifier(source[:], q[2], q[3]),
            Amplifier(source[:], q[3], q[4]),
            Amplifier(source[:], q[4], q[0], last_value),
        ]
        
        for amp in amplifiers:
            amp.start()
            
        amplifiers[4].join()

        max_output = max(max_output, last_value[0])

    print(max_output)


def get_combinations(elements):
    if not elements:
        yield []

    for i in range(len(elements)):
        for combination in get_combinations(elements[:i] + elements[i+1:]):
            yield [elements[i]] + combination



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
