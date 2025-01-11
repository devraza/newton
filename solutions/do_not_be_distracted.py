import string, itertools

t = int(input())

for _ in range(t):
    n = int(input())
    s = input()

    filtered = []

    s = ''.join(i for i, _ in itertools.groupby(s))

    check = True
    for i in string.ascii_uppercase:
        if s.count(i) > 1:
            check = False
            break
    if check:
        print("YES")
    else:
        print("NO")
