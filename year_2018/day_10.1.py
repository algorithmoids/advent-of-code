import re
import time


def main():
    points = get_data()
    i = 0
    while True:
        i += 1
        min_point = 1000
        max_point = -1000
        for point in points:
            point[0] += point[2]
            point[1] += point[3]

            min_point = min(min_point, point[1])
            max_point = max(max_point, point[1])

        if max_point - min_point < 10:
            min_y = min([x[0] for x in points])
            max_y = max([x[0] for x in points])
            min_x = min([x[1] for x in points])
            max_x = max([x[1] for x in points])
            screen = [['.'] * (max_y - min_y + 1) for x in range(max_x - min_x + 1)]
            print(points)
            for point in points:
                screen[point[1] - min_x][point[0] - min_y] = '#'

            for row in screen:
                print(''.join(row))

            print('to wait: {}'.format(i))

            return


def get_data():
    pattern = re.compile(r'position=<\s*(\S+),\s*(\S+)> velocity=<\s*(\S+),\s*(\S+)>')
    fp = open('input/day_10.1.txt')
    points = []
    for row in fp:
        points.append(list(map(int, pattern.search(row).groups())))

    return points

if __name__ == '__main__':
    start = time.monotonic()
    print(main())
    time_spent = time.monotonic() - start
    print('took: {:.3f} s'.format(time_spent))
