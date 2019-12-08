import time


def main():
    source = data()
    layer_size = 25 * 6
    image = ['2'] * layer_size

    i = 0
    while True:
        layer = source[i:i+layer_size]
        if not layer:
            break

        image = [l if p == '2' else p for p, l in zip(image, layer)]
        i += layer_size

    for i in range(0, layer_size, 25):
        print(''.join(['##' if x == '1' else '  ' for x in image[i: i+25]]))

def data():
    return open('input/day_8.txt').read().strip()


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
