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
    done = set()

    workers = [(0, '')] * 5
    time_spent = 0

    while free or done or [x for x in workers if x[1] != '']:
        sorted_free = sorted(free)
        doing = 0
        for i in range(len(workers)):
            if doing == len(sorted_free):
                break
            if workers[i][1] == '':
                workers[i] = (61 + ord(sorted_free[doing]) - ord('A'), sorted_free[doing])
                free.remove(sorted_free[doing])
                doing += 1


        busy = [x for x in workers if x[1] != '']
        if busy and (len(busy) == len(workers) or not free):
            next_finishing = min(busy)
            worker_id = workers.index(next_finishing)
            time_spent += next_finishing[0]
            done.add(next_finishing[1])
            new_workers = []
            for work, element in workers:
                if element:
                    new_workers.append((work - next_finishing[0], element))
                else:
                    new_workers.append((0, ''))
            workers = new_workers
            workers[worker_id] = (0, '')

        next_node = min(done)
        done.remove(next_node)

        for dependent in start_nodes[next_node]:
            end_nodes[dependent].remove(next_node)
            if len(end_nodes[dependent]) == 0:
                free.add(dependent)

    return time_spent


def get_data() -> List[List[int]]:
    parser = re.compile(r'Step (\w) must be finished before step (\w) can begin.')
    for row in open('input/day_7.1.txt'):
        yield parser.search(row).groups()


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
