import gc
import time
from collections import defaultdict

players = 455
marbels = 7122300
answer = 384288

score = defaultdict(int)


class Marble:
    def __init__(self, value, cw=None, ccw=None):
        self.value = value
        self.cw = cw
        self.ccw = ccw


class Circle:
    def __init__(self):
        self.current = Marble(0)
        self.current.cw = self.current
        self.current.ccw = self.current

    def insert(self, value):
        marble = Marble(value, cw=self.current.cw, ccw=self.current)
        self.current.cw.ccw = marble
        self.current.cw = marble
        self.current = marble

    def pop(self):
        value = self.current.value
        self.current = self.current.cw
        self.current.ccw.ccw.cw = self.current
        return value

    def change_current(self, shift):
        if shift == 1:
            self.current = self.current.cw
        if shift == -7:
            self.current = self.current.ccw.ccw.ccw.ccw.ccw.ccw.ccw

    def __str__(self):
        value = self.current.value
        values = [str(self.current.value)]
        printing = self.current.cw

        while printing.value != value:
            values.append(str(printing.value))
            printing = printing.cw

        return ' '.join(values)


def main():
    circle = Circle()
    for i in range(1, marbels):
        if i % 23 == 0:
            elf = i % players
            circle.change_current(-7)
            score[elf] += i + circle.pop()
        else:
            circle.change_current(1)
            circle.insert(i)

    print(max(score.values()))


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print('took: {:.3f} s'.format(time_spent))
