import time
from collections import defaultdict


def main():
    initial = '#.#####.#.#.####.####.#.#...#.......##..##.#.#.#.###..#.....#.####..#.#######.#....####.#....##....#'
    # initial = '#..#.#..##......###...###'
    pots = {i for i, x in enumerate(initial) if x == '#'}

    changes = {i: x for i, x in enumerate(initial)}
    rules = get_rules()

    for i in range(50000000000):
        to_check = set([x for pod in changes.keys() for x in [pod-2, pod-1, pod, pod+1, pod+2]])
        changes = {}
        for pot in to_check:
            near_pots = ''.join(['#' if (pot+x) in pots else '.' for x in range(-2, 3)])
            new_value = rules[near_pots]
            if pot in pots and new_value == '.':
                changes[pot] = '.'
            elif pot not in pots and new_value == '#':
                changes[pot] = '#'

        for pot, change in changes.items():
            if change == '.':
                pots.remove(pot)
            else:
                pots.add(pot)


        print(min(pots), ''.join(['#' if i in pots else '.' for i in range(min(pots), max(pots) + 1)]))
    print(sum(pots))

def get_rules():
    code = '''##.## => .
#.#.. => .
..... => .
##..# => #
###.. => #
.##.# => .
..#.. => #
##.#. => #
.##.. => .
#..#. => .
###.# => #
.#### => #
.#.## => .
#.##. => #
.###. => #
##### => .
..##. => .
#.#.# => .
...#. => #
..### => .
.#.#. => #
.#... => #
##... => #
.#..# => #
#.### => #
#..## => #
....# => .
####. => .
#...# => #
#.... => .
...## => .
..#.# => #'''

    rules_dict = {}
    for row in code.split('\n'):
        rules_dict[row[:5]] = row[-1]

    return rules_dict

def get_rules2():
    code = '''...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #'''

    rules_dict = defaultdict(lambda: '.')
    for row in code.split('\n'):
        rules_dict[row[:5]] = row[-1]

    return rules_dict


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print('took: {:.3f} s'.format(time_spent))
