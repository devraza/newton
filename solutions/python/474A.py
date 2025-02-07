shift = input()
s = input()

rows = [
    "qwertyuiop",
    "asdfghjkl;",
    "zxcvbnm,./",
]

construct = ""

for i in range(len(s)):
    for j in rows:
        row = list(j)
        if s[i] in row:
            if shift == "R":
                change = -1
            else:
                change = +1

            construct += row[row.index(s[i])+change]

print(construct)
