import time

serial = 9221


def main():
    grid = []
    for i in range(1, 301):
        row = []
        for j in range(1, 301):
            rack = j + 10
            value = (((rack * i + serial) * rack // 100) % 10) - 5
            row.append(value)

        grid.append(row)

    max_cell = 0
    coordinates = None
    for i in range(300):
        for j in range(300):
            max_size = min(300 - i, 300 - j)
            s = 0

            for size in range(1, max_size):
                for k in range(size):
                    s += grid[j + size - 1][i + k]
                for k in range(size - 1):
                    s += grid[j + k][i + size - 1]

                if s > max_cell:
                    max_cell = s
                    coordinates = (j, i, size)
    return coordinates[1] + 1, coordinates[0] + 1, coordinates[2]


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print('took: {:.3f} s'.format(time_spent))
