import string

s = input()

l = 0
h = 0

for i in list(s):
    if i in string.ascii_uppercase:
        h += 1
    else:
        l += 1

if h > l:
    print(s.upper())
else:
    print(s.lower())
