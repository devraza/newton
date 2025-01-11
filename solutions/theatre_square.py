from math import ceil

n, m, a = [int(x) for x in input().split(' ')]

val1 = ceil(n/a)
val2 = ceil(m/a)

print(val1*val2)
