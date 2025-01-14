t = int(input())

for _ in range(t):
    n = list(input())

    count = 0
    for i in n:
        count += int(i)
    print(count)
