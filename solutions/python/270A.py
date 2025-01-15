import sys

t = int(input())

for _ in range(t):
    a = int(input())
    exterior = 180-a
    if 360 % exterior == 0:
        print("YES")
    else:
        print("NO")
