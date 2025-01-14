import string,sys

s = list(input())

option_one = True
for i in string.ascii_lowercase:
    if i in s:
        option_one = False

option_two = True
if len(s) > 1:
    for i in range(1, len(s)):
        for j in string.ascii_lowercase:
            if s[i] == j:
                option_two = False
comp = len(s) == 1 or option_two

if not (comp or option_one):
    print(''.join(c for c in s))
    sys.exit()

for idx, i in enumerate(s):
    if i in string.ascii_uppercase:
        s[idx] = i.lower()
    else:
        s[idx] = i.upper()

print(''.join(c for c in s))
