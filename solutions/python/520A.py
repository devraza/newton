import string

n = input()
s = input().lower()

check = True
for i in string.ascii_lowercase:
    if s.count(i) == 0:
        check = False

if check:
    print("YES")
else:
    print("NO")
