#!/usr/bin/python3
from time import time

def ackermann(m: int, n: int):
    if m == 0:
        return n + 1
    elif n == 0:
        return ackermann(m - 1, 1)
    else:
        return ackermann(m - 1, ackermann(m, n - 1))


def test():
    values = []
    for _ in range(0, 500):
        start = time()
        ackermann(3, 3)
        end = time()
        values.append(end - start)
    return values

values = test()
values.sort()
total = sum(values) * 1000
average = total / len(values)
median = values[len(values) // 2] * 1000
amplitude = values[len(values) - 1] * 1000 - values[0] * 1000

print(f'Total: {total}ms ; Average: {average}ms ; Median: {median}ms ; Amplitude: {amplitude}ms')
