n = int(input())

coordinates = [0, 0, 0]

for _ in range(n):
    x, y, z = [int(d) for d in input().split(' ')]

    coordinates[0] += x
    coordinates[1] += y
    coordinates[2] += z

check = True
for i in coordinates:
    if i != 0:
        check = False

if check:
    print("YES")
else:
    print("NO")
