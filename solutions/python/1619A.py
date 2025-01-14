t = int(input())

for _ in range(t):
    s = input()
    if len(s) % 2 != 0:
        print("NO")
    elif s[0:len(s)//2] == s[len(s)//2:len(s)]:
        print("YES")
    else:
        print("NO")
