t = int(input())

for _ in range(t):
    x, y, n=[int(x) for x in input().split()]
    k=n % x
    if k > y:
        print(n-k+y)
    elif k == y:
        print(n)
    else:
        print(n-k-x+y)
