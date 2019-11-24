import re
import time
from collections import defaultdict
from typing import List


def main():
    edges = list(get_data())

    start_nodes = defaultdict(list)
    end_nodes = defaultdict(list)

    for end, start in edges:
        end_nodes[start].append(end)
        start_nodes[end].append(start)

    free = set(start_nodes.keys()).difference(end_nodes.keys())

    order = []
    while free:
        next_node = min(free)
        free.remove(next_node)
        order.append(next_node)

        for dependent in start_nodes[next_node]:
            end_nodes[dependent].remove(next_node)
            if len(end_nodes[dependent]) == 0:
                free.add(dependent)

    return ''.join(order)



def get_data() -> List[List[int]]:
    parser = re.compile(r'Step (\w) must be finished before step (\w) can begin.')
    for row in open('input/day_7.1.txt'):
        yield parser.search(row).groups()


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
