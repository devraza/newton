import sys

n = int(input())

laptops = []

for _ in range(n):
    laptops.append([int(d) for d in input().split(' ')])

laptops = sorted(laptops, key=lambda x: x[0])

if any(laptops[i][1] > laptops[i+1][1] for i in range(n-1)):
    print('Happy Alex')
else:
    print('Poor Alex')
