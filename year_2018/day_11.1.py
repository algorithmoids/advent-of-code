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
        print(row)

    max_cell = 0
    coordinates = None
    for i in range(300 - 3):
        for j in range(300 - 3):
            s = 0
            for k in range(3):
                for l in range(3):
                    s += grid[i + k][j + l]
            if s > max_cell:
                max_cell = s
                coordinates = (j, i)
                print(s)
    return coordinates


if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print('took: {:.3f} s'.format(time_spent))
