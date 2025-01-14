import re

s = input()

matches = re.findall(r"[a-z]*h[a-z]*e[a-z]*l[a-z]*l[a-z]*o", s)

if len(matches) >= 1:
    print("YES")
else:
    print("NO")
