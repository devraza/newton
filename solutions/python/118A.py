s = list(input().lower())

i = ["a", "e", "i", "o", "u", "y"]
s = [c for c in s if c not in i]

for idx, i in enumerate(s):
    s[idx] = f".{i}"

print(''.join(c for c in s))
