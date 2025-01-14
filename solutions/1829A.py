t = int(input())

s = "codeforces"
for _ in range(t):
    indices = 0
    case = input()
    for i in range(len(case)):
        if case[i] != s[i]:
            indices += 1
    print(indices)
