t = int(input())

for _ in range(t):
    n = input()
    a = [int(n) for n in input().split(' ')]

    print(max(a)-min(a))
