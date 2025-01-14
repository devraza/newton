from itertools import permutations

t = int(input())

perms = [''.join(x) for x in permutations("Timur", 5)]

for _ in range(t):
    n = int(input())
    s = input()

    if n != 5:
        print("NO")
    elif s in perms:
        print("YES")
    else:
        print("NO")
