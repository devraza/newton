n, k = list(map(int, input().split(' ')))

for _ in range(k):
    if str(n)[-1] != "0":
        n -= 1
    else:
        n = n // 10

print(n)
