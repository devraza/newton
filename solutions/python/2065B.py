t = int(input())

for _ in range(t):
    w = list(input())

    adjacent = False

    for i in range(1, len(w)):
        if w[i] == w[i-1]:
            adjacent = True

        if len(w) <= 1:
            break

    if not adjacent:
        print(len(w))
        continue

    print("1")
