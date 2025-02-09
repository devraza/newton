t = int(input())

for _ in range(t):
    n, m = map(int, input().split(' '))

    a = []

    for _ in range(n):
        a.append(list(map(int, input().split(' '))))

    a = sorted(a, key=lambda x: sum(x))[::-1]

    concat = []

    for i in a:
        for j in i:
            concat.append(j)

    total = 0

    for idx, i in enumerate(concat):
        total += (n*m-idx)*i

    print(total)
