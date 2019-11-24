import re
import time
from collections import defaultdict


def main():
    guards = defaultdict(list)

    current_guard = None
    sleep_start = None
    for action, value in get_input():
        if action == 'guard':
            current_guard = value
        if action == 'sleep':
            sleep_start = value
        if action == 'wake':
            guards[current_guard].append([sleep_start, value])

    minutes = max([[count_minutes(times), guard] for guard, times in guards.items()])
    print(minutes)
    return minutes[0][1] * minutes[1]

def count_minutes(times):
    minutes = [0] * 60

    for start, end in times:
        for i in range(end - start):
            minutes[start + i] += 1
    minute, count = max(enumerate(minutes), key=lambda x: x[1])
    return count, minute



def get_input():
    re_guard = re.compile(r' #(\d+)')
    re_minute = re.compile(r':(\d+)')

    f = open('input/day_4.1.txt')
    log = sorted(f.readlines())

    for row in log:
        if 'Guard' in row:
            yield 'guard', int(re_guard.search(row).groups()[0])
        if 'falls asleep' in row:
            yield 'sleep', int(re_minute.search(row).groups()[0])
        if 'wakes up' in row:
            yield 'wake', int(re_minute.search(row).groups()[0])


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
