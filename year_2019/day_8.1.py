import time


def main():
    image = data()
    layer_size = 25 * 6
    fewer_zeroes = layer_size
    fewer_zeroes_layer = None
    i = 0
    while True:
        layer = image[i:i + layer_size]
        if not layer:
            break

        zeroes = len([x for x in layer if x == '0'])
        if zeroes < fewer_zeroes:
            fewer_zeroes = zeroes
            fewer_zeroes_layer = i

        i += layer_size

    print(len([x for x in image[fewer_zeroes_layer:fewer_zeroes_layer + layer_size] if x == '1'])
          * len([x for x in image[fewer_zeroes_layer:fewer_zeroes_layer + layer_size] if x == '2']))


def data():
    return open('input/day_8.txt').read().strip()


if __name__ == '__main__':
    start = time.monotonic()
    main()
    time_spent = time.monotonic() - start
    print(f'took: {time_spent:.3f} s')
