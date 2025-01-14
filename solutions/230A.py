import sys

s, n = [int(d) for d in input().split(' ')]

dragons = []

for _ in range(n):
    dragons.append([int(d) for d in input().split(' ')])

dragons = sorted(dragons, key=lambda x: x[0])

for i in dragons:
    if s > i[0]:
        s += i[1]
    else:
        print("NO")
        sys.exit()
print("YES")
