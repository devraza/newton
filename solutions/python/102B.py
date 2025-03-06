n = str(input())

count = 0

while len(n) != 1:
    total = map(int, list(n))
    n = str(sum(total))
    count += 1

print(count)
