t = int(input())

for _ in range(t):
    digits = sorted([int(x) for x in input().split(' ')])

    if min(digits) == digits[1]:
        print(digits[2])
    else:
        print(digits[0])
