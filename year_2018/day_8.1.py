import re
import time
from collections import defaultdict
from typing import List


def main():
    tree = get_data()

    return sum_metadata(tree)[0]


def sum_metadata(tree):
    nodes = tree[0]
    metadata = tree[1]
    offset = 2
    tree_sum = 0
    for i in range(nodes):
        branch_sum, branch_offset = sum_metadata(tree[offset:])
        offset += branch_offset
        tree_sum += branch_sum
    for i in range(metadata):
        tree_sum += tree[offset + i]

    return tree_sum, offset + metadata


def get_data() -> List[int]:
    return list(map(int, open('input/day_8.1.txt').read().strip().split(' ')))


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
