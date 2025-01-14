n = int(input())

mishka = 0
chris = 0

for _ in range(n):
    m, c = [int(x) for x in input().split(' ')]

    if m > c:
        mishka += 1
    elif c > m:
        chris += 1

if mishka > chris:
    print("Mishka")
elif chris > mishka:
    print("Chris")
else:
    print("Friendship is magic!^^")
