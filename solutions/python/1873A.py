t = int(input())

for _ in range(t):
    s = input()
    s = [s[0], s[1], s[2]]
    original = s[:]

    check = False
    s[0], s[2] = s[2], s[0]
    if ''.join(c for c in s) == "abc" or ''.join(c for c in original) == "abc":
        check = True
    else:
        s = original[:]
        s[0], s[1] = s[1], s[0]
        if ''.join(c for c in s) == "abc":
            check = True
        else:
            s = original[:]
            s[1], s[2] = s[2], s[1]
            if ''.join(c for c in s) == "abc":
                check = True
    if check:
        print("YES")
    else:
        print("NO")
