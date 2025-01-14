n = int(input())
a = [int(c) for c in input().split(' ')]

vals = []

def find(a):
    count = 0
    first = 0
    second = sum(a)

    for i in a:
        first += i
        second -= i

        count += 1

        if first > second:
            vals.append(count)
            break
    if len(vals) >= 2:
        return
    find(a[::-1])
find(sorted(a))

print(min(vals))
