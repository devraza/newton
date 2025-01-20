import re

s = input()
s = re.sub(r"WUB*", " ", s)
s = ' '.join(s.split(' '))

print(s)
