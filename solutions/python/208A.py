import re

s = input()
s = re.sub(r"WUB", " ", s)
s = ' '.join([c for c in s.split(' ') if c != ''])

print(s)
