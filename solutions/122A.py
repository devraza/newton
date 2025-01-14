import itertools, sys

lucky4 = [int(''.join(x for x in c)) for c in itertools.product('74', repeat=4)]
lucky3 = [int(''.join(x for x in c)) for c in itertools.product('74', repeat=3)]
lucky2 = [int(''.join(x for x in c)) for c in itertools.product('74', repeat=2)]
lucky1 = [4,7]

lucky = lucky4+lucky3+lucky2+lucky1

n = int(input())

for i in lucky:
    if n % i == 0:
        print("YES")
        sys.exit()
print("NO")
