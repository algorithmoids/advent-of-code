import time
from typing import List


def main():
    order = {x: y for y, x in data()}

    s = path_to_com(order, 'SAN')
    u = path_to_com(order, 'YOU')

    print(len(set(s).difference(u)) + len(set(u).difference(s)) - 2)


def path_to_com(order, element):
    path = [element]
    while element != 'COM':
        element = order[element]
        path.append(element)

    return path


def data() -> List[List[str]]:
    input_text = open('input/day_6.txt').read()
    return [x.split(')') for x in input_text.strip().split('\n')]


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
